use crate::nats_actor::*;

impl NatsSubActor {
    pub fn subscribe(subj: &'static str) -> Self {
        let actor: NatsActor = NatsActor::new();
        //println!("actor {:?}", actor);
        let mut client = actor.client;
        let s1 = client.subscribe(subj, None).unwrap();
        Self {
            client: client,
            subject: subj,
            channel: s1,
        }
    }

    pub fn event(events: Events) {
        for event in events {
            let msg = String::from_utf8(event.msg).unwrap();
            println!("client msg {:?}", msg)
        }
    }
}
