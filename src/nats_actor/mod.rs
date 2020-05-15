use nats::*;
pub mod actor_sub;
pub mod request;
pub mod util;

#[derive(Debug)]
pub struct NatsActor {
    client: Client,
}

#[derive(Debug)]
pub struct NatsSubActor {
    client: Client,
    subject: &'static str,
    channel: Channel,
}

impl NatsActor {
    pub fn new() -> Self {
        let cluster = vec![dotenv!("NATs_URL")];
        let client = Client::new(cluster).unwrap();
        Self {
            client: client,
        }
    }
}
