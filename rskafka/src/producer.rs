pub use kafka_protocol::{CodecError, KafkaRpc, KafkaRpcType};
use tokio::net::{TcpStream};
use kafka_api::{KafkaRequest, api::ResponseBody};
use kafka_api::{api::{MetadataRequest, RequestBody}, apikey::ApiKeys};
use crate::*;

pub struct Producer {

}

impl Producer {
    pub async fn new(config: ProducerConfig) -> Result<Producer, RsError> {
        Ok(Producer {})
    }

    pub async fn produce(&mut self) -> tokio::io::Result<()> {
        todo!();
        Ok(())
    }
}
