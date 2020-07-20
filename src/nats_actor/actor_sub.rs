use crate::nats_actor::*;

impl NatsSubActor {
    pub fn subscribe(subj: &'static str) -> Self {
        let actor: NatsActor = NatsActor::new();
        //println!("subject {}", subj);
        let mut client = actor.client;
        let s1 = client.subscribe(subj, Some("queue")).unwrap();
        Self {
            client: client,
            subject: subj,
            channel: s1,
        }
    }
    pub fn wait_event(client: Client) {
        let mut client = client;
        client.wait().unwrap();
        let event = client
            .events()
            .find(|event| event.subject == "topic.order")
            .unwrap();
        let msg = String::from_utf8(event.msg).unwrap();
        println!("msg {:?}", msg)
    }

    pub fn event(client: Client) {
        let mut client = client;
        for event in client.events() {
            let msg = String::from_utf8(event.msg).unwrap();
            println!("msg {:?}", msg)
        }
    }
}
