use crate::nats_actor::*;

impl NatsPubActor {
    pub fn publish(subj: &'static str) -> Self {
        let actor: NatsActor = NatsActor::new();
        let mut client = actor.client;
        client.publish(subj, b"I need help!").unwrap();
        Self {
            client: client,
            subject: subj,
        }
    }
}
