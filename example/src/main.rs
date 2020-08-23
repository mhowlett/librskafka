use rskafka::*;
use std::io::Write;


#[tokio::main]
async fn main() -> Result<(), RsError> {
    // setup logging. todo, configurable etc.
    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env().format(move |buf, rec| {
        let t = start.elapsed().as_secs_f32();
        writeln!(buf, "{:.03} [{}] - {}", t, rec.level(),rec.args())
    }).init();

    let mut config = ConsumerConfig::new();
    config.bootstrap_servers = vec!["localhost:9092".into()];
    config.client_id = "u-bute_consumer".into();

    // does not complete until at least one broker connection has been established.
    let conusmer = Consumer::new(config).await?;

    // let mut c = Consumer::new().await
    // c.subscribe().await?;
    Ok(())
}