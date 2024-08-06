use anyhow::Result;
use clap::Parser;

use jz_action::{
    api::{
        self,
        server::start_rpc_server,
    },
    dbrepo::{
        MongoConfig,
        MongoMainDbRepo,
        MongoRunDbRepo,
    },
    driver::kube::KubeDriver,
    utils::StdIntoAnyhowResult,
};
use kube::Client;
use std::{
    path::Path,
    str::FromStr,
};
use tokio::{
    fs,
    io::AsyncWriteExt,
    select,
    signal::unix::{
        signal,
        SignalKind,
    },
    task::JoinSet,
    time::Instant,
};
use tokio_util::sync::CancellationToken;
use tracing::{
    error,
    info,
    Level,
};

use jz_action::job::job_mgr::JobManager;

#[derive(Debug, Parser)]
#[command(
    name = "jz-action-backend",
    version = "0.0.1",
    author = "Author Name <github.com/GitDataAI/jz-action>",
    about = "jz-action backend"
)]

struct Args {
    #[arg(short, long, default_value = "INFO")]
    log_level: String,

    #[arg(short, long, default_value = "localhost:45131")]
    listen: String,

    #[arg(short, long)]
    mongo_url: String,

    #[arg(short, long, default_value = "/app/tmp")]
    tmp_path: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(&args.log_level)?)
        .try_init()
        .anyhow()?;
    let mut join_set: JoinSet<Result<()>> = JoinSet::new();
    let token = CancellationToken::new();

    let mongo_cfg = MongoConfig::new(args.mongo_url.clone());
    let client = Client::try_default().await.unwrap();
    let job_manager =
        JobManager::<MongoRunDbRepo, MongoConfig>::new(client, mongo_cfg.clone()).await?;
    let db_repo = MongoMainDbRepo::new(mongo_cfg, "backend").await?;
    let server = start_rpc_server(&args.listen, db_repo, job_manager).unwrap();
    let handler = server.handle();
    {
        //listen unix socket
        let token = token.clone();
        let handler = handler.clone();
        join_set.spawn(async move {
            info!("start ipc server {}", &args.listen);
            tokio::spawn(server);
            select! {
                _ = token.cancelled() => {
                    handler.stop(true).await;
                   info!("rpc server stopped");
                   return Ok(());
                }
            };
        });
    }

    {
        //catch signal
        let _ = tokio::spawn(async move {
            let mut sig_term = signal(SignalKind::terminate()).unwrap();
            let mut sig_int = signal(SignalKind::interrupt()).unwrap();
            select! {
                _ = sig_term.recv() => info!("Recieve SIGTERM"),
                _ = sig_int.recv() => info!("Recieve SIGTINT"),
            };
            token.cancel();
        });
    }

    while let Some(Err(err)) = join_set.join_next().await {
        error!("exit spawn {err}");
    }
    info!("gracefully shutdown");
    Ok(())
}
