use anyhow::{
    anyhow,
    Result,
};
use clap::Parser;
use compute_unit_runner::ipc::{
    self,
    IPCClient,
    SubmitOuputDataReq,
};
use itertools::Itertools;
use jiaoziflow::{
    core::db::{
        DataFlag,
        TrackerState,
        KEEP_DATA,
        TRANSPARENT_DATA,
    },
    utils::StdIntoAnyhowResult,
};
use jiaozifs_client_rs::{
    apis::{
        self,
        configuration::Configuration,
    },
    models::RefType,
};
use std::{
    path::Path,
    str::FromStr,
    time::Instant,
};
use tokio::{
    fs,
    select,
    signal::unix::{
        signal,
        SignalKind,
    },
    task::JoinSet,
};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{
    info,
    Level,
};

#[derive(Debug, Parser)]
#[command(
    name = "jz_reader",
    version = "0.0.1",
    author = "Author Name <github.com/GitDataAI/jiaoziflow>",
    about = "embed in k8s images. "
)]

struct Args {
    #[arg(short, long, default_value = "INFO")]
    log_level: String,

    #[arg(short, long, default_value = "/unix_socket/compute_unit_runner_d")]
    unix_socket_addr: String,

    #[arg(short, long, default_value = "/app/tmp")]
    tmp_path: String,

    #[arg(long, default_value = "64")]
    batch_size: usize,

    #[arg(long)]
    jiaozifs_url: String,

    #[arg(long)]
    username: String,

    #[arg(long)]
    password: String,

    #[arg(long)]
    owner: String,

    #[arg(long)]
    repo: String,

    #[arg(long)]
    ref_type: String,

    #[arg(long)]
    ref_name: String,

    #[arg(long, default_value = "*")]
    pattern: String,

    #[arg(long, help = "output directory labeling as name")]
    enable_directory_labeling: bool,

    #[arg(long)]
    labels_name: Option<String>,
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
        join_set.spawn(async move { read_jz_fs(args).await });
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

async fn read_jz_fs(args: Args) -> Result<()> {
    let ref_type = match args.ref_type.as_str() {
        "branch" => Ok(RefType::Branch),
        "wip" => Ok(RefType::Wip),
        "tag" => Ok(RefType::Tag),
        "commit" => Ok(RefType::Commit),
        val => Err(anyhow!("ref type not correct {}", val)),
    }?;

    let configuration = Configuration {
        base_path: args.jiaozifs_url,
        basic_auth: Some((args.username, Some(args.password))),
        ..Default::default()
    };

    info!("try to list files");
    let file_paths = apis::objects_api::get_files(
        &configuration,
        &args.owner,
        &args.repo,
        &args.ref_name,
        ref_type,
        Some(&args.pattern),
    )
    .await?;

    info!("get files {} in jizozifs", file_paths.len());
    let client = ipc::IPCClientImpl::new(args.unix_socket_addr);
    let tmp_path = Path::new(&args.tmp_path);
    let status = client.status().await.anyhow()?;

    if status.state == TrackerState::Finish {
        return Ok(());
    }

    //write label
    if args.enable_directory_labeling {
        let labels = get_directory_labes(&file_paths);
        let id = args
            .labels_name
            .unwrap_or_else(|| status.node_name.clone() + "-label");
        let output_dir = tmp_path.join(&id).join("labels.txt");
        if let Some(parent) = output_dir.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(&output_dir, labels.join("\n")).await?;
        client
            .submit_output(SubmitOuputDataReq::new(
                &id,
                1,
                DataFlag::new_from_bit(KEEP_DATA | TRANSPARENT_DATA),
                1,
            ))
            .await
            .anyhow()?;
        info!("flush label files {:?}", output_dir);
    }

    //write data
    for batch in file_paths.chunks(args.batch_size) {
        //create temp output directory
        let id = uuid::Uuid::new_v4().to_string();
        let output_dir = tmp_path.join(&id);
        fs::create_dir_all(output_dir.clone()).await?;

        let instant = Instant::now();
        //read file from jzfs and write to output directory
        for path in batch {
            let mut byte_stream = apis::objects_api::get_object(
                &configuration,
                &args.owner,
                &args.repo,
                &args.ref_name,
                path.as_str(),
                ref_type,
                None,
            )
            .await?;

            let file_path = output_dir.as_path().join(path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).await?;
            }
            let mut tmp_file = fs::File::create(file_path).await?;
            while let Some(item) = byte_stream.next().await {
                tokio::io::copy(&mut item.unwrap().as_ref(), &mut tmp_file)
                    .await
                    .unwrap();
            }
        }
        info!(
            "read a batch files {} spent {:?}",
            batch.len(),
            instant.elapsed()
        );

        //submit directory after completed a batch
        client
            .submit_output(SubmitOuputDataReq::new(
                &id,
                batch.len() as u32,
                DataFlag::new_from_bit(0),
                0,
            ))
            .await
            .anyhow()?;
        info!(
            "flush batch files {} spent {:?}",
            batch.len(),
            instant.elapsed()
        );
    }
    // read all files
    client.finish().await.unwrap();
    info!("read jiaozifs successfully");
    Ok(())
}

fn get_directory_labes(file_paths: &[String]) -> Vec<String> {
    file_paths
        .iter()
        .filter_map(|path| {
            let path = Path::new(path);
            let x: Vec<_> = path.components().collect();
            if x.len() < 2 {
                return None;
            }
            return Some(x[0].as_os_str().to_string_lossy().to_string());
        })
        .unique()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_directory_labes() {
        let paths = vec![
            "aaa".to_string(),
            "aaa.txt".to_string(),
            "a/a.txt".to_string(),
            "b/a.txt".to_string(),
            "c/a.txt".to_string(),
            "a/c/da.txt".to_string(),
            "d/e/f/a.txt".to_string(),
        ];

        let labels = get_directory_labes(&paths);
        assert_eq!(labels, vec!["a", "b", "c", "d"])
    }
}
