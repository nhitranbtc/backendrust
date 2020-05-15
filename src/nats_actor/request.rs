use crate::nats_actor::*;

pub fn make_request(subj: &'static str) {
    let mut client = NatsActor::new().client;
    let inbox = client.make_request(subj, b"I need help!").unwrap();
}

pub fn test_make_request() {
    let subj = "hel.please";
    let mut client = NatsSubActor::subscribe(subj).client;
    make_request(subj);
    NatsSubActor::event(client.events());
}

