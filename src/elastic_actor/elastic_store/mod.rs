pub mod commands;
pub mod queries;

use elastic::{
    client::SyncClient,
    error::Error,
};

pub struct Client {
    io: SyncClient,
}

impl Client {
    pub fn new(address: &'static str) -> Result<Self, Error> {
        let client = SyncClient::builder().static_node(address).build()?;
        Ok(Client {io: client})
    }
}
