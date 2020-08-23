pub use kafka_protocol::{CodecError, KafkaRpc, KafkaRpcType};
use crate::cluster::*;
use std::{cell::RefCell, sync::Arc};
use crate::rs_error::*;
use crate::consumer_config::*;
use tokio::task;
use task::JoinHandle;


pub struct Consumer {
    cluster: Arc<RefCell<Cluster>>,
    cg_task: JoinHandle<()>,
}


impl Consumer {

    async fn consumer_group_run()  {
        loop {

        }
    }

    pub async fn new(config: ConsumerConfig) -> Result<Consumer, RsError> {
        let cluster = Arc::new(RefCell::new(Cluster::new(config.bootstrap_servers, &config.client_id)));
        // cluster.bootstrap(false).await?;

        let cg_task = task::spawn(Consumer::consumer_group_run());

        Ok(Consumer {
            cluster,
            cg_task
        })
    }

    pub async fn close(&mut self) -> Result<(), RsError> {

        Ok(())
    }

    pub async fn subscribe(&mut self) -> Result<(), RsError> {

        Ok(())
    }

    pub async fn assign(&mut self) -> Result<(), RsError> {

        Ok(())
    }

    pub async fn incremental_assign(&mut self) -> Result<(), RsError> {

        Ok(())
    }

}
