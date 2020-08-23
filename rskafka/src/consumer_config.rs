
pub struct ConsumerConfig {
    pub bootstrap_servers: Vec<String>,
    pub client_id: String,
    pub group_id: String,
}

impl ConsumerConfig {
    pub fn new() -> ConsumerConfig {
        ConsumerConfig {
            bootstrap_servers: vec!["localhost:9092".to_owned()],
            client_id: "rskafka".to_owned(),
            group_id: "rskafka-group".to_owned(),
        }
    }
}