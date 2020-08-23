use tokio::net::{TcpStream};
use kafka_api::{KafkaRequest, KafkaResponse};
use kafka_transport::*;
use kafka_transport::error::TransportError;
use crate::rs_error::RsLocalError;
use kafka_api::apikey::ApiKeys;

/// Encapsulates a connection to a broker. Does not encapsulate retry logic,
/// that happens at a higher level.
pub struct Connection {
    address: Option<String>,
    transport: Option<Transport<KafkaResponse, KafkaRequest>>,
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            address: None,
            transport: None
        }
    }

    pub fn address(&self) -> Result<&String, RsLocalError> { self.address.as_ref().ok_or(RsLocalError::Fail) }

    pub async fn open(&mut self, address: &str) -> std::io::Result<()> {
        let socket = TcpStream::connect(address).await?;  // timeout.
        self.transport = Some(kafka_transport::new_client_transport(socket));
        self.address = Some(address.to_string());
        Ok(())
    }

    pub async fn write_request(&mut self, request: KafkaRequest) -> Result<(), RsLocalError> {
        match &mut self.transport {
            None => Err(RsLocalError::Transport),
            Some(transport) => {
                match transport.write_request(request).await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        match e {
                            TransportError::TooLarge => Err(RsLocalError::TooLarge), // equivalent librdkafka codes?
                            TransportError::Codec(_) => Err(RsLocalError::Codec),
                            TransportError::Io(_) => {
                                self.transport = None;
                                Err(RsLocalError::Transport)
                            }
                        }
                    }
                }
            }
        }
    }

    pub async fn read_response(&mut self, api_key: ApiKeys, api_version: i16) -> Result<KafkaResponse, RsLocalError> {
        match &mut self.transport {
            None => Err(RsLocalError::Transport),
            Some(transport) => {
                match transport.read_response(api_key, api_version).await {
                    Ok(r) => Ok(r),
                    Err(e) => {
                        match e {
                            TransportError::TooLarge => Err(RsLocalError::TooLarge), // equivalent librdkafka codes?
                            TransportError::Codec(_) => Err(RsLocalError::Codec),
                            TransportError::Io(_) => {
                                self.transport = None;
                                Err(RsLocalError::Transport)
                            }
                        }
                    }
                }
            }
        }
    }
    
}
