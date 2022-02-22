use chrono::{DateTime, Utc};

enum Executor {
    Spawn(String),
    WarehouseDep(String),
    Deliverer(String),
}

pub enum State {
    Created(Executor, DateTime<Utc>),
    Distributed(Executor, DateTime<Utc>),
    Delivery(Executor, DateTime<Utc>),
    Finished,
}

pub struct StateMachine {
    state: State,
}

impl StateMachine {
    fn new() -> Self {
        Self {
            state: State::Created(Executor::Spawn(""into()), D)
        }
    }
}