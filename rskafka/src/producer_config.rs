
pub struct ProducerConfig {
    pub bootstrap_servers: Vec<String>,
    pub client_id: String,
}

impl ProducerConfig {
    pub fn new() -> ProducerConfig {
        ProducerConfig {
            bootstrap_servers: vec!["localhost:9092".to_owned()],
            client_id: "rskafka".to_owned(),
        }
    }
}