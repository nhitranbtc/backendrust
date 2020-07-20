use crate::nats_actor::*;

pub fn make_request(subj: &'static str) {
    let mut client = NatsActor::new().client;
    let inbox = client.make_request(subj, b"I need help!").unwrap();
}

pub fn test_make_request() {
    let subj = "topic.order";
    let actor: NatsSubActor = NatsSubActor::subscribe(subj);
    let mut client = actor.client;
    make_request(subj);
    NatsSubActor::event(client);
}

