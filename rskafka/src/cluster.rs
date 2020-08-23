use crate::broker::*;
use crate::connection::*;
pub use kafka_protocol::{CodecError, KafkaRpc, KafkaRpcType};
use kafka_api::{KafkaRequest, api::ResponseBody};
use kafka_api::{api::{MetadataRequest, RequestBody}, apikey::ApiKeys};
use crate::rs_error::{RsError};
use log::*;

pub struct Cluster {
    bootstrap_correlation_id: i32,
    bootstrap_servers: Vec<String>,
    client_id: String,
    brokers: Vec<Broker>,
}

impl Cluster {
    pub fn new(bootstrap_servers: Vec<String>, client_id: &str) -> Cluster
    {
        Cluster {
            bootstrap_correlation_id: 10000,
            bootstrap_servers,
            client_id: client_id.to_string(),
            brokers: vec![]
        }
    }

    /// "Bootstrap" the cluster via an address in bootstrap servers. Following completion of
    /// this method, there will be one connected broker only (others will be opened
    /// as required), unless open_all_connections is true, in which case connections to all
    /// brokers will be attempted to be established.
    ///
    /// The bootstrap connection used to access the cluster is not held open if it doesn't
    /// correspond to an advertised listener (obtained via a metadata request).
    ///
    /// Use this method only if there are no connected brokers in the cluster.
    pub async fn bootstrap(&mut self, open_all_connections: bool) -> Result<(), RsError> {

        // if at least on broker is alive, then no need to bootstrap.
        for broker in self.brokers.iter() {
            if broker.is_connected() { return Ok(()) }
        }

        for bootstrap_server in &self.bootstrap_servers {
            let mut connection = Connection::new();
            match connection.open(bootstrap_server).await {
                Err(_) => {
                    warn!("Could not establish connection to bootstrap server: {}", bootstrap_server);
                    continue
                },
                Ok(_) => {}
            }

            // Metadata request v0 is supported by all broker versions (no need to do an api
            // version request), and is sufficient to determine all brokers in the cluster.
            // However, there's a problem serializing the request, so use v1 for now, which
            // should be supported by practically everything.. TODO: use v0.
            let request = KafkaRequest::new(ApiKeys::Metadata, 
                1, self.bootstrap_correlation_id, &self.client_id,
                RequestBody::MetadataRequest(MetadataRequest {
                    topics: None, allow_auto_topic_creation: false,
                }));
            self.bootstrap_correlation_id += 1;
            match connection.write_request(request).await {
                Err(e) => {
                    warn!("Metadata request failed: {}", e);
                    continue
                },
                Ok(_) => {}
            }
            let response = match connection.read_response(ApiKeys::Metadata, 1 as i16).await {
                Err(e) => {
                    error!("Failed to read metadata response: {}", e);
                    continue
                },
                Ok(r) => {
                    match r.body {
                        ResponseBody::MetadataResponse(m) => m,
                        _ => {
                            error!("Metadata response body has unexpected type");
                            continue
                        }
                    }
                }
            };

            self.brokers.clear();
            for broker_metadata in response.brokers {
                let mut broker = Broker::new(broker_metadata.node_id);
                match broker.open(&format!("{}:{}", broker_metadata.host, broker_metadata.port)).await {
                    Err(e) => {
                        warn!("Connection to broker {}:{}, node_id: {}: {} could not be established", broker_metadata.host, broker_metadata.port, broker_metadata.node_id, e);
                        continue;
                    },
                    Ok(_) => {}
                }
                info!("Connected to {}:{}, node_id: {}", broker_metadata.host, broker_metadata.port, broker_metadata.node_id);
            }

            if !open_all_connections {
                break;
            }
        }

        Ok(())
    }

    pub fn get_random_connected_broker(&mut self) -> Option<&mut Broker> {
        for broker in &mut self.brokers {
            if broker.is_connected() {
                return Some(broker);
            }
        }
        None
    }
}
