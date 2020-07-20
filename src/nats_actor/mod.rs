use nats::*;
pub mod actor_sub;
pub mod actor_pub;
pub mod request;
pub mod util;

pub type ActorClient = Client;

#[derive(Debug)]
pub struct NatsActor {
    pub client: Client,
}

#[derive(Debug)]
pub struct NatsSubActor {
    pub client: Client,
    subject: &'static str,
    channel: Channel,
}

#[derive(Debug)]
pub struct NatsPubActor {
    pub client: Client,
    subject: &'static str,
}

impl NatsActor {
    pub fn new() -> Self {
        let cluster = vec![dotenv!("NATs_URL")];
        let client = Client::new(cluster).unwrap();
        //println!("NATs connection");
        Self {
            client: client,
        }
    }
}
