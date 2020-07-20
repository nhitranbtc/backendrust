use crate::nats_actor::*;
use crate::order_topic::*;
use bastion::prelude::*;
use std::sync::Arc;
use std::time::Duration;

pub fn subscribe() {
    Bastion::init();
    Bastion::supervisor(input_supervisor)
        //.and_then(|_| Bastion::supervisor(map_supervisor))
        .expect("Couldn't create supervisor chain.");
    Bastion::start();
}

pub fn receive_event() {
    Bastion::init();
    Bastion::supervisor(map_supervisor).expect("Couldn't create supervisor chain.");
    Bastion::start();
}

fn input_supervisor(supervisor: Supervisor) -> Supervisor {
    supervisor.children(|children| sub_group(children))
}

// Supervisor for actors in map group
fn map_supervisor(supervisor: Supervisor) -> Supervisor {
    let restart_strategy = RestartStrategy::default()
        .with_restart_policy(RestartPolicy::Tries(3))
        .with_actor_restart_strategy(ActorRestartStrategy::LinearBackOff {
            timeout: Duration::from_secs(1),
        });
    supervisor
        .with_restart_strategy(restart_strategy)
        .children(|children| process_group(children))
}

fn sub_group(children: Children) -> Children {
    let subj = "topic.order";
    children
        .with_name("subscribe")
        .with_redundancy(1)
        .with_exec(move |ctx: BastionContext| async move {
            println!("[Input] Worker started!");
            let group_name = "Processing".to_string();
            let target = BroadcastTarget::Group(group_name);
            let actor: NatsSubActor = NatsSubActor::subscribe(subj);
            let mut client = actor.client;
            for event in client.events() {
                let msg = String::from_utf8(event.msg).unwrap();
                //println!("msg {:?}", msg);
                ctx.broadcast_message(target.clone(), msg);
            }
            //NatsSubActor::event(client);
            Ok(())
        })
}

fn process_group(children: Children) -> Children {
    children
        .with_name("process")
        .with_redundancy(10)
        .with_dispatcher(Dispatcher::with_type(DispatcherType::Named(
            "Processing".to_string(),
        )))
        .with_exec(move |ctx: BastionContext| async move {
            println!("[Processing] raw_message {:?}", ctx.recv().await?);
            //println!("[Processing] Worker started!");
            msg! {ctx.recv().await?,
            //We received the message from other actor wrapped in Arc<T>
            // Let's unwrap it and do regular matching
            raw_message: Arc<SignedMessage> => {
                //println!("[Processing] raw_message {:?}", raw_message);
                let message = Arc::try_unwrap(raw_message).unwrap();
                msg! { message,
                ref data: String  => {
                    //println!("[Processing] Worker #{:?} ref received '{}'", ctx.current().id(), data);
                };
                _:_ => { println!("[Processing] Worker nomatch");
                    ()};
                }
            };
            _:_ => { println!("[Processing] Worker nomatch raw_message");
                ()};
            }
            Ok(())
        })
}
