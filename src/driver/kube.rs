use super::{
    Driver,
    NodeStatus,
    PipelineController,
    PodStauts,
    UnitHandler,
};
use crate::{
    core::{
        db::{
            Graph,
            GraphRepo,
            JobDbRepo,
            Node,
            NodeType,
            TrackerState,
        },
        AccessMode,
        ComputeUnit,
        StorageOptions,
    },
    dag::Dag,
    dbrepo::MongoRunDbRepo,
    utils::{
        k8s_helper::get_pod_status,
        IntoAnyhowResult,
        StdIntoAnyhowResult,
    },
};
use anyhow::{
    anyhow,
    Result,
};
use chrono::prelude::*;
use futures::future::try_join_all;
use handlebars::{
    Context,
    Handlebars,
    Helper,
    Output,
    RenderContext,
    RenderError,
};
use k8s_metrics::v1beta1 as metricsv1;
use k8s_openapi::api::{
    apps::v1::StatefulSet,
    core::v1::{
        Namespace,
        PersistentVolumeClaim,
        Pod,
        Service,
    },
};
use kube::{
    api::{
        DeleteParams,
        ListParams,
        PostParams,
    },
    runtime::reflector::Lookup,
    Api,
    Client,
};
use serde::Serialize;
use std::{
    collections::HashMap,
    default::Default,
    marker::PhantomData,
};
use tokio_retry::{
    strategy::ExponentialBackoff,
    Retry,
};
use tracing::{
    debug,
    error,
    warn,
};

pub struct KubeHandler<R>
where
    R: JobDbRepo,
{
    pub(crate) client: Client,
    pub(crate) node_name: String,
    pub(crate) namespace: String,
    pub(crate) stateset_name: String,
    pub(crate) claim_name: String,
    pub(crate) _service_name: String,
    pub(crate) db_repo: R,
}

impl<R> UnitHandler for KubeHandler<R>
where
    R: JobDbRepo,
{
    fn name(&self) -> &str {
        &self.node_name
    }

    async fn status(&self) -> Result<NodeStatus> {
        let statefulset_api: Api<StatefulSet> =
            Api::namespaced(self.client.clone(), &self.namespace);
        let claim_api: Api<PersistentVolumeClaim> =
            Api::namespaced(self.client.clone(), &self.namespace);
        let pods_api: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);
        let metrics_api: Api<metricsv1::PodMetrics> =
            Api::<metricsv1::PodMetrics>::namespaced(self.client.clone(), &self.namespace);

        let statefulset = statefulset_api.get(&self.stateset_name).await.anyhow()?;
        let selector = statefulset
            .spec
            .as_ref()
            .unwrap()
            .selector
            .match_labels
            .as_ref()
            .expect("set in template")
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join(",");

        let list_params = ListParams::default().labels(&selector);
        let pods = pods_api.list(&list_params).await.anyhow()?;

        let pvc = claim_api.get(&self.claim_name).await.anyhow()?;
        let cap = pvc
            .status
            .unwrap()
            .capacity
            .unwrap()
            .get("storage")
            .map(|cap| cap.0.clone())
            .unwrap_or_default();

        let db_node = self.db_repo.get_node_by_name(&self.node_name).await?;
        let data_count = self
            .db_repo
            .count(&self.node_name, &Vec::new(), None)
            .await?;
        let mut node_status = NodeStatus {
            name: self.node_name.clone(),
            state: db_node.state,
            data_count,
            replicas: statefulset
                .spec
                .as_ref()
                .and_then(|spec| spec.replicas)
                .unwrap_or_default() as u32,
            storage: cap,
            pods: HashMap::new(),
        };

        for pod in pods {
            let pod_name = pod.metadata.name.as_ref().expect("set in template");
            let phase = get_pod_status(&pod);

            let metrics = metrics_api
                .get(pod_name)
                .await
                .anyhow()
                .map_err(|err| {
                    warn!("get metrics fail {err} {}", pod_name);
                    err
                })
                .unwrap_or_default();

            let mut cpu_sum = 0.0;
            let mut memory_sum = 0;
            for container in metrics.containers.iter() {
                cpu_sum += container
                    .usage
                    .cpu()
                    .map_err(|err| {
                        error!("cpu not exit {err}");
                        err
                    })
                    .unwrap_or(0.0);
                memory_sum += container
                    .usage
                    .memory()
                    .map_err(|err| {
                        error!("cpu not exit {err}");
                        err
                    })
                    .unwrap_or(0);
            }
            let pod_status = PodStauts {
                state: phase,
                cpu_usage: cpu_sum,
                memory_usage: memory_sum,
            };
            node_status.pods.insert(pod_name.clone(), pod_status);
        }
        Ok(node_status)
    }

    async fn start(&self) -> Result<()> {
        self.db_repo
            .update_node_by_name(&self.node_name, TrackerState::Ready)
            .await
    }

    async fn pause(&mut self) -> Result<()> {
        todo!()
    }

    async fn restart(&mut self) -> Result<()> {
        todo!()
    }

    async fn stop(&mut self) -> Result<()> {
        todo!()
    }
}

pub struct KubePipelineController<R>
where
    R: JobDbRepo,
{
    _client: Client,
    _db_repo: R,
    topo_sort_nodes: Vec<String>,
    handlers: HashMap<String, KubeHandler<R>>,
}

impl<R> KubePipelineController<R>
where
    R: JobDbRepo,
{
    fn new(repo: R, client: Client, topo_sort_nodes: Vec<String>) -> Self {
        Self {
            _db_repo: repo,
            topo_sort_nodes,
            _client: client,
            handlers: Default::default(),
        }
    }
}

impl<R> PipelineController for KubePipelineController<R>
where
    R: JobDbRepo,
{
    type Output = KubeHandler<R>;

    async fn start(&self) -> Result<()> {
        try_join_all(self.handlers.iter().map(|handler| handler.1.start()))
            .await
            .map(|_| ())
    }
    //todo use iter
    fn nodes_in_order(&self) -> Result<Vec<String>> {
        Ok(self.topo_sort_nodes.clone())
    }

    async fn get_node(&self, id: &str) -> Result<&KubeHandler<R>> {
        self.handlers.get(id).anyhow("id not found")
    }

    async fn get_node_mut(&mut self, id: &str) -> Result<&mut KubeHandler<R>> {
        self.handlers.get_mut(id).anyhow("id not found")
    }
}

fn join_array(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h.param(0);

    match param {
        None => Ok(()),
        Some(args) => {
            let args = args.value().as_array().unwrap();
            let args_str: Vec<String> = args
                .iter()
                .map(|v| "\"".to_owned() + v.as_str().unwrap() + "\"")
                .collect();
            let rendered = args_str.join(",").to_string();
            out.write(rendered.as_ref())?;
            Ok(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct KubeOptions {
    db_url: String,
    storage: StorageOptions,
}

impl Default for KubeOptions {
    fn default() -> Self {
        Self {
            db_url: "".to_string(),
            storage: StorageOptions {
                class_name: Some("jz-flow-fs".to_string()),
                capacity: Some("1Gi".to_string()),
                access_mode: Some(AccessMode::ReadWriteMany),
            },
        }
    }
}

fn merge_storage_options(opt1: &StorageOptions, opt2: &StorageOptions) -> StorageOptions {
    let mut opts = StorageOptions {
        class_name: opt1.class_name.clone(),
        capacity: opt1.capacity.clone(),
        access_mode: opt1.access_mode.clone(),
    };
    if let Some(class_name) = opt2.class_name.as_ref() {
        opts.class_name = Some(class_name.clone());
    }
    if let Some(capacity) = opt2.capacity.as_ref() {
        opts.capacity = Some(capacity.clone());
    }
    if let Some(access_mode) = opt2.access_mode.as_ref() {
        opts.access_mode = Some(access_mode.clone());
    }
    opts
}

impl KubeOptions {
    pub fn set_db_url(mut self, db_url: &str) -> Self {
        self.db_url = db_url.to_string();
        self
    }

    pub fn set_storage_class(mut self, class_name: &str) -> Self {
        self.storage.class_name = Some(class_name.to_string());
        self
    }

    pub fn set_capacity(mut self, capacity: &str) -> Self {
        self.storage.capacity = Some(capacity.to_string());
        self
    }

    pub fn set_access_mode(mut self, mode: AccessMode) -> Self {
        self.storage.access_mode = Some(mode);
        self
    }
}

#[derive(Clone)]
pub struct KubeDriver<R>
where
    R: JobDbRepo,
{
    reg: Handlebars<'static>,
    client: Client,

    options: KubeOptions,
    _phantom_data: PhantomData<R>,
}

impl<R> KubeDriver<R>
where
    R: JobDbRepo,
{
    pub async fn new(client: Client, options: KubeOptions) -> Result<KubeDriver<R>> {
        let mut reg = Handlebars::new();
        reg.register_template_string("claim", include_str!("kubetpl/claim.tpl"))?;

        reg.register_template_string("statefulset", include_str!("kubetpl/statefulset.tpl"))?;
        reg.register_template_string("service", include_str!("kubetpl/service.tpl"))?;
        reg.register_helper("join_array", Box::new(join_array));
        Ok(KubeDriver {
            reg,
            client,
            options,
            _phantom_data: PhantomData,
        })
    }

    async fn ensure_namespace_exit_and_clean(client: &Client, ns: &str) -> Result<()> {
        let namespace = Namespace {
            metadata: kube::api::ObjectMeta {
                name: Some(ns.to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let namespaces: Api<Namespace> = Api::all(client.clone());
        // Create the namespace
        if namespaces.get(ns).await.is_ok() {
            let _ = namespaces
                .delete(ns, &DeleteParams::default())
                .await
                .map(|_| ())
                .map_err(|e| anyhow!("{}", e.to_string()));
            let retry_strategy = ExponentialBackoff::from_millis(1000).take(20);
            Retry::spawn(retry_strategy, || async {
                match namespaces.get(ns).await {
                    Ok(_) => Err(anyhow!("expect deleted")),
                    Err(err) => {
                        if err.to_string().contains("not found") {
                            Ok(())
                        } else {
                            Err(anyhow!("retry"))
                        }
                    }
                }
            })
            .await?;
        }
        namespaces
            .create(&PostParams::default(), &namespace)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!("{}", e.to_string()))
    }
}

#[derive(Serialize)]
struct ClaimRenderParams {
    storage: StorageOptions,
    name: String,
}

#[derive(Serialize)]
struct NodeRenderParams<'a> {
    node: &'a ComputeUnit,
    log_level: &'a str,
    db_url: &'a str,
    run_id: &'a str,
}

impl<R> Driver for KubeDriver<R>
where
    R: JobDbRepo,
{
    #[allow(refining_impl_trait)]
    async fn deploy(
        &self,
        run_id: &str,
        graph: &Dag,
    ) -> Result<KubePipelineController<MongoRunDbRepo>> {
        Self::ensure_namespace_exit_and_clean(&self.client, run_id).await?;

        let db_url = self.options.db_url.clone() + "/" + run_id;
        let repo = MongoRunDbRepo::new(db_url.as_str())
            .await
            .map_err(|err| anyhow!("create database fail {err}"))?;
        let statefulset_api: Api<StatefulSet> = Api::namespaced(self.client.clone(), run_id);
        let claim_api: Api<PersistentVolumeClaim> = Api::namespaced(self.client.clone(), run_id);
        let service_api: Api<Service> = Api::namespaced(self.client.clone(), run_id);

        // insert global record
        let cur_tm = Utc::now().timestamp();
        let graph_record = Graph {
            graph_json: graph.raw.clone(),
            created_at: cur_tm,
            updated_at: cur_tm,
        };
        repo.insert_global_state(&graph_record).await?;
        let topo_sort_nodes = graph.topo_sort_nodes();
        let mut pipeline_ctl =
            KubePipelineController::new(repo.clone(), self.client.clone(), topo_sort_nodes);
        for node in graph.iter() {
            if node.spec.command.is_empty() {
                return Err(anyhow!("{} dont have command", &node.name));
            }

            let data_unit_render_args = NodeRenderParams {
                node,
                db_url: db_url.as_str(),
                log_level: "debug",
                run_id,
            };
            let up_nodes = graph.get_incomming_nodes(&node.name);
            let down_nodes = graph.get_outgoing_nodes(&node.name);

            // apply nodes
            let claim_string = self.reg.render(
                "claim",
                &ClaimRenderParams {
                    storage: merge_storage_options(&self.options.storage, &node.spec.storage),
                    name: node.name.clone() + "-node-claim",
                },
            )?;
            debug!("rendered clam string {}", claim_string);
            let claim: PersistentVolumeClaim = serde_json::from_str(&claim_string)?;
            let claim_deployment = claim_api.create(&PostParams::default(), &claim).await?;

            let statefulset_string = self.reg.render("statefulset", &data_unit_render_args)?;
            debug!("rendered unit string {}", statefulset_string);

            let unit_statefulset: StatefulSet = serde_json::from_str(&statefulset_string)?;
            let unit_statefulset = statefulset_api
                .create(&PostParams::default(), &unit_statefulset)
                .await?;

            // compute unit only receive data from channel
            let outgoing_node_streams = down_nodes
                .iter()
                .map(|node_name| {
                    format!(
                        "http://{}-service.{}.svc.cluster.local:80",
                        node_name, run_id
                    )
                })
                .collect::<Vec<_>>();

            let node_record = Node {
                node_name: node.name.clone(),
                state: TrackerState::Init,
                node_type: NodeType::CoputeUnit,
                up_nodes: up_nodes.iter().map(|v| v.to_string()).collect(),
                outgoing_streams: outgoing_node_streams,
                created_at: cur_tm,
                updated_at: cur_tm,
            };

            repo.insert_node(&node_record).await?;

            let service_string = self.reg.render("service", node)?;
            debug!("rendered unit service config {}", service_string);

            let unit_service: Service = serde_json::from_str(service_string.as_str())?;
            let unit_service = service_api
                .create(&PostParams::default(), &unit_service)
                .await?;

            let handler = KubeHandler {
                node_name: node.name.clone(),
                client: self.client.clone(),
                namespace: run_id.to_string().clone(),
                stateset_name: unit_statefulset
                    .name()
                    .expect("set name in template")
                    .to_string(),
                claim_name: claim_deployment
                    .name()
                    .expect("set name in template")
                    .to_string(),
                _service_name: unit_service
                    .name()
                    .expect("set name in template")
                    .to_string(),
                db_repo: repo.clone(),
            };

            pipeline_ctl.handlers.insert(node.name.clone(), handler);
        }
        Ok(pipeline_ctl)
    }

    #[allow(refining_impl_trait)]
    async fn attach(
        &self,
        run_id: &str,
        graph: &Dag,
    ) -> Result<KubePipelineController<MongoRunDbRepo>> {
        let db_url = self.options.db_url.clone() + "/" + run_id;
        let repo = MongoRunDbRepo::new(db_url.as_str())
            .await
            .map_err(|err| anyhow!("create database fail {err}"))?;
        let statefulset_api: Api<StatefulSet> = Api::namespaced(self.client.clone(), run_id);
        let claim_api: Api<PersistentVolumeClaim> = Api::namespaced(self.client.clone(), run_id);
        let service_api: Api<Service> = Api::namespaced(self.client.clone(), run_id);

        let topo_sort_nodes = graph.topo_sort_nodes();
        let mut pipeline_ctl =
            KubePipelineController::new(repo.clone(), self.client.clone(), topo_sort_nodes);
        for node in graph.iter() {
            // apply nodes
            let claim_deployment = claim_api
                .get((node.name.clone() + "-node-claim").as_str())
                .await?;
            let unit_statefulset = statefulset_api
                .get((node.name.clone() + "-statefulset").as_str())
                .await?;

            let unit_service = service_api
                .get((node.name.clone() + "-service").as_str())
                .await?;

            let handler = KubeHandler {
                node_name: node.name.clone(),
                client: self.client.clone(),
                namespace: run_id.to_string().clone(),
                stateset_name: unit_statefulset
                    .name()
                    .expect("set name in template")
                    .to_string(),
                claim_name: claim_deployment
                    .name()
                    .expect("set name in template")
                    .to_string(),
                _service_name: unit_service
                    .name()
                    .expect("set name in template")
                    .to_string(),
                db_repo: repo.clone(),
            };

            pipeline_ctl.handlers.insert(node.name.clone(), handler);
        }
        Ok(pipeline_ctl)
    }

    async fn clean(&self, ns: &str) -> Result<()> {
        let client: Client = Client::try_default().await?;
        let namespaces: Api<Namespace> = Api::all(client.clone());
        if namespaces.get(ns).await.is_ok() {
            namespaces
                .delete(ns, &DeleteParams::default())
                .await
                .map(|_| ())
                .map_err(|e| anyhow!("{}", e.to_string()))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dbrepo::MongoRunDbRepo;
    use local_ip_address::local_ip;
    use mongodb::Client as MongoClient;
    use std::env;
    use tracing_subscriber;

    #[tokio::test]
    async fn test_render() {
        env::set_var("RUST_LOG", "DEBUG");
        tracing_subscriber::fmt::init();
        let json_str = r#"
        {
          "name": "example",
          "version": "v1",
          "dag": [
           {
              "name": "make-article",
              "spec": {
                "image": "gitdatateam/make_article:latest",
                "command":"/make_article",
                "args": ["--log-level=debug", "--total-count=5"]
              }
            },   {
              "name": "copy-in-place",
              "node_type": "ComputeUnit",
              "dependency": [
                "make-article"
              ],
              "spec": {
                "image": "gitdatateam/copy_in_place:latest",
                "command":"/copy_in_place",
                "replicas": 3,
                "args": ["--log-level=debug"]
              }
            },
            {
              "name": "list-files",
              "node_type": "ComputeUnit",
              "dependency": [
                "copy-in-place"
              ],
              "spec": {
                "image": "gitdatateam/list_files:latest",
                "command":"/list_files",
                "replicas": 3,
                "args": ["--log-level=debug"]
              }
            }
          ]
        }
                        "#;
        let dag = Dag::from_json(json_str).unwrap();
        let my_local_ip = local_ip().unwrap();
        let db_url = format!("mongodb://{}:27017", my_local_ip);
        let client = MongoClient::with_uri_str(db_url.to_string() + "/ntest")
            .await
            .unwrap();
        client.database("ntest").drop().await.unwrap();

        let client = Client::try_default().await.unwrap();

        let options = KubeOptions::default().set_db_url(&db_url);
        let kube_driver = KubeDriver::<MongoRunDbRepo>::new(client, options)
            .await
            .unwrap();
        kube_driver.deploy("ntest", &dag).await.unwrap();
        //    kube_driver.clean("ntest").await.unwrap();
    }
}
