use anyhow::Result;
use clap::Parser;
use compute_unit_runner::ipc::{
    self,
    IPCClient,
    SubmitOuputDataReq,
};
use jiaoziflow::{
    core::db::{
        DataFlag,
        TrackerState,
    },
    utils::StdIntoAnyhowResult,
};
use random_word::Lang;
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
    info,
    Level,
};

#[derive(Debug, Parser)]
#[command(
    name = "dummyu_in",
    version = "0.0.1",
    author = "Author Name <github.com/GitDataAI/jiaoziflow>",
    about = "embed in k8s images. "
)]

struct Args {
    #[arg(short, long, default_value = "INFO")]
    log_level: String,

    #[arg(short, long, default_value = "/unix_socket/compute_unit_runner_d")]
    unix_socket_addr: String,

    #[arg(short, long, default_value = "0")]
    total_count: u32,

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
    let mut join_set = JoinSet::new();
    let token = CancellationToken::new();

    {
        let token = token.clone();
        join_set.spawn(async move { make_article(token, args).await });
    }

    {
        //catch signal
        tokio::spawn(async move {
            let mut sig_term = signal(SignalKind::terminate()).unwrap();
            let mut sig_int = signal(SignalKind::interrupt()).unwrap();
            select! {
                _ = sig_term.recv() => info!("Recieve SIGTERM"),
                _ = sig_int.recv() => info!("Recieve SIGTINT"),
            };
            token.cancel();
        });
    }

    nodes_sdk::monitor_tasks(&mut join_set).await
}

async fn make_article(token: CancellationToken, args: Args) -> Result<()> {
    let client = ipc::IPCClientImpl::new(args.unix_socket_addr);
    let tmp_path = Path::new(&args.tmp_path);
    let count_file = tmp_path.join("number.txt");
    let mut count = if fs::try_exists(&count_file).await? {
        let count_str = fs::read_to_string(&count_file).await?;
        str::parse::<u32>(count_str.as_str())?
    } else {
        0
    };

    loop {
        if args.total_count > 0 && count >= args.total_count {
            info!("exit pod because work has done");
            if client.status().await.unwrap().state == TrackerState::Finish {
                return Ok(());
            }

            client.finish().await.unwrap();
            return Ok(());
        }

        if token.is_cancelled() {
            return Ok(());
        }

        let instant = Instant::now();
        let id = uuid::Uuid::new_v4().to_string();
        let output_dir = tmp_path.join(&id);
        fs::create_dir_all(output_dir.clone()).await?;
        for _ in 0..30 {
            let folder_name = random_word::gen(Lang::En);
            let file_name = random_word::gen(Lang::En);
            let file_path = output_dir
                .as_path()
                .join(folder_name)
                .join(file_name.to_string() + ".txt");
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).await?;
            }
            let mut tmp_file = fs::File::create(file_path).await?;
            for _ in 0..100 {
                let word = random_word::gen(Lang::En).to_string() + "\n";
                let _ = tmp_file.write(word.as_bytes()).await.unwrap();
            }
        }

        info!("generate new data spent {:?}", instant.elapsed());
        //submit directory after completed a batch
        client
            .submit_output(SubmitOuputDataReq::new(
                &id,
                30,
                DataFlag::new_from_bit(0),
                0,
            ))
            .await
            .anyhow()?;
        info!("submit new data({count}) {:?}", instant.elapsed());
        count += 1;
        fs::write(&count_file, count.to_string()).await.unwrap();
    }
}
