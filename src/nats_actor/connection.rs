use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

pub fn connect_cluster() {
    let cluster = vec!["nats://ruser:T0pS3cr3t@127.0.0.1"];
    let mut client = nats::Client::new(cluster).unwrap();

    let subj1 = "topic/test1";
    let subj2 = "topic/test2";
    let subj3 = "topic/test3";
    let chan1 = "channel1";
    let chan2 = "channel2";

    let p = Person {
        first_name: "nhi".to_owned(),
        last_name: "tran".to_owned(),
        age: 27,
    };
    let vec = serde_json::to_vec(&p).unwrap();

    //client.subscribe(&subj1, None).unwrap();
    //client.subscribe(&subj2, None).unwrap();
    //client.subscribe(&subj3, None).unwrap();
    client.subscribe(&chan1, None).unwrap();

    client.publish(&subj1, &vec).unwrap();
    client.publish(&subj2, "Hello world!".as_bytes()).unwrap();
    client.publish(&subj3, "Msg random".as_bytes()).unwrap();

    // let find_event = client
    //     .events()
    //     .find(|event| event.subject == "topic/test1")
    //     .unwrap();

    // let p: Person = serde_json::from_slice(&find_event.msg).unwrap();
    // println!("event msg {:?}", p);

    let s = client.subscribe(chan2, Some("queue")).unwrap();
    client.unsubscribe(s).unwrap();
    client.make_request(chan1, b"test").unwrap();
    //let event = client.wait().unwrap();
    //println!("wait {:?}", event);
    //client.subscribe("channel.*", None).unwrap();
    client.publish(chan1, b"test1").unwrap();
    client.publish(chan1, b"test2").unwrap();
    client.publish(chan1, b"test3").unwrap();

    for event in client.events() {
        let subject = event.subject.as_str();
        match subject {
            "topic/test1" => {
                // pub fn serde_json::from_slice(bytes: &'a [u8]) -> Self: convert Vec<u8> to Self
                let p: Person = serde_json::from_slice(&event.msg).unwrap();
                println!("subject {:?}; msg {:?}", subject, p);
            }
            // pub fn String::from_utf8_lossy(v: &[u8]) -> Cow<str>: convert Vec<u8> to string
            "topic/test2" => println!(
                "subject {:?}; msg {:?}",
                subject,
                String::from_utf8_lossy(&event.msg)
            ),

            _ => println!(
                "subject {:?}; msg {:?}",
                subject,
                String::from_utf8_lossy(&event.msg)
            ),
        }
    }
}
