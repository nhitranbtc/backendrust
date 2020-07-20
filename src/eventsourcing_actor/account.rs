use crate::elastic_actor::model;
use eventsourcing::*;

use crate::elastic_actor::elastic_store::{
    commands::{Document},
    Client,
};

type AccountData = model::mytype::NewMyType;

const DOMAIN_VERSION: &str = "1.0";

impl AggregateState for AccountData {
    fn generation(&self) -> i64 {
        self.generation
    }
}

#[event_type_version(DOMAIN_VERSION)]
#[event_source("events://github.com/pholactery/eventsourcing/samples/location")]
#[derive(Serialize, Deserialize, Debug, Clone, Event)]

enum AccountEvent {
    AccountUpdated { title: String },
}

struct Account;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum AccountCommand {
    UpdateAccount { title: String },
}

impl Aggregate for Account {
    type Event = AccountEvent;
    type Command = AccountCommand;
    type State = AccountData;

    fn apply_event(state: &Self::State, evt: &Self::Event) -> Result<Self::State> {
        let ld = match evt {
            AccountEvent::AccountUpdated { title } => AccountData {
                title: title.to_string(),
                generation: state.generation + 1,
            },
        };
        Ok(ld)
    }
    fn handle_command(_state: &Self::State, cmd: &Self::Command) -> Result<Vec<Self::Event>> {
        // SHOULD DO: validate state and command
        let evt = match *&cmd {
            AccountCommand::UpdateAccount { title } => AccountEvent::AccountUpdated {
                title: title.to_string(),
            },
        };

        // if validation passes...
        Ok(vec![evt])
    }
}

pub fn account() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://localhost:9200")?;
    let old_state = AccountData {
        title: "Title".to_string(),
        generation: 0,
    };
    client.index(json!(old_state))?;

    let update = AccountCommand::UpdateAccount {
        title: "Title".to_string(),
    };

    // First, handle a command to get an event vector
    let res = Account::handle_command(&old_state, &update)?;

    // Second, apply the events to get a new state
    let state = Account::apply_all(&old_state, &res)?;
    client.index(json!(state))?;

    println!("Original state: {:?}", old_state);
    println!("Post-process state: {:?}", state);

    Ok(())
}
