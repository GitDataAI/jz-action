use anyhow::Result;
use jz_flow::{
    core::db::JobDbRepo,
    network::{
        common::Empty,
        datatransfer::{
            data_stream_server::DataStream,
            MediaDataBatchResponse,
        },
    },
};
use std::sync::Arc;
use tokio::sync::{
    oneshot,
    RwLock,
};
use tonic::{
    Request,
    Response,
    Status,
};

use super::channel_tracker::ChannelTracker;

pub struct ChannelDataStream<R>
where
    R: JobDbRepo,
{
    pub(crate) program: Arc<RwLock<ChannelTracker<R>>>,
}

#[tonic::async_trait]
impl<R> DataStream for ChannelDataStream<R>
where
    R: JobDbRepo,
{
    async fn transfer_media_data(
        &self,
        req: Request<MediaDataBatchResponse>,
    ) -> Result<Response<Empty>, Status> {
        let send_tx = {
            let program = self.program.read().await;
            if program.incoming_tx.is_none() {
                return Err(Status::internal("not ready"));
            }
            program.incoming_tx.as_ref().unwrap().clone()
        };

        let (tx, rx) = oneshot::channel::<Result<()>>();
        let req: MediaDataBatchResponse = req.into_inner();
        if let Err(err) = send_tx.send((req, tx)).await {
            return Err(Status::from_error(Box::new(err)));
        }

        match rx.await {
            Ok(Ok(_)) => Ok(Response::new(Empty {})),
            Ok(Err(err)) => Err(Status::internal(err.to_string())),
            Err(err) => Err(Status::from_error(Box::new(err))),
        }
    }

    async fn request_media_data(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<MediaDataBatchResponse>, Status> {
        let send_tx = {
            let program = self.program.read().await;
            if program.request_tx.is_none() {
                return Err(Status::internal("not ready"));
            }
            program.request_tx.as_ref().unwrap().clone()
        };

        let (tx, rx) = oneshot::channel::<Result<Option<MediaDataBatchResponse>>>();
        if let Err(err) = send_tx.send(((), tx)).await {
            return Err(Status::from_error(Box::new(err)));
        }

        match rx.await {
            Ok(Ok(Some(resp))) => Ok(Response::new(resp)),
            Ok(Ok(None)) => Err(Status::not_found("no avaiable data")),
            Ok(Err(err)) => Err(Status::from_error(err.into())),
            Err(err) => Err(Status::from_error(Box::new(err))),
        }
    }
}