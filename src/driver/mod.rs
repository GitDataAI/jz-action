pub mod kube;

use crate::{
    core::db::TrackerState,
    dag::Dag,
};
use anyhow::Result;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    collections::HashMap,
    future::Future,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PodStauts {
    pub state: String,
    pub cpu_usage: f64,
    pub memory_usage: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatus {
    pub name: String,
    pub state: TrackerState,
    pub data_count: usize,
    pub replicas: u32,
    pub storage: String,
    pub pods: HashMap<String, PodStauts>,
}

pub trait UnitHandler: Send {
    fn name(&self) -> &str;

    fn start(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    //pause graph running for now
    fn status(&self) -> impl Future<Output = Result<NodeStatus>> + Send;

    //pause graph running for now
    fn pause(&mut self) -> impl Future<Output = Result<()>> + Send;

    //restart paused graph
    fn restart(&mut self) -> impl Future<Output = Result<()>> + Send;

    //stop resource about this graph
    fn stop(&mut self) -> impl Future<Output = Result<()>> + Send;
}
pub trait PipelineController: Send {
    type Output: UnitHandler;

    fn start(&self) -> impl std::future::Future<Output = Result<()>> + Send;

    fn nodes_in_order(&self) -> Result<Vec<String>>;

    fn get_node(&self, id: &str)
        -> impl std::future::Future<Output = Result<&Self::Output>> + Send;

    fn get_node_mut(
        &mut self,
        id: &str,
    ) -> impl std::future::Future<Output = Result<&mut Self::Output>> + Send;
}

pub trait Driver: 'static + Clone + Send + Sync {
    //deploy graph to cluster
    fn deploy(
        &self,
        namespace: &str,
        graph: &Dag,
    ) -> impl Future<Output = Result<impl PipelineController>> + Send;

    //attach cluster in cloud with graph
    fn attach(
        &self,
        namespace: &str,
        graph: &Dag,
    ) -> impl Future<Output = Result<impl PipelineController>> + Send;

    //clean all resource about this graph
    fn clean(&self, namespace: &str) -> impl Future<Output = Result<()>> + Send;
}
