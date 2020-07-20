use crate::nats_actor::*;
use bastion::prelude::*;

pub fn publish() {
    Bastion::init();
    Bastion::supervisor(input_supervisor)
        .expect("Couldn't create supervisor chain.");
    Bastion::start();
}

fn input_supervisor(supervisor: Supervisor) -> Supervisor {
    supervisor.children(|children| pub_group(children))
}

fn pub_group(children: Children) -> Children {
    let subj = "topic.order";
    children
        .with_name("publish")
        .with_redundancy(10)
        .with_exec(move |_ctx: BastionContext| async move {
            NatsPubActor::publish(subj);
            //println!("[Published msg]");
            Ok(())
        })
}



