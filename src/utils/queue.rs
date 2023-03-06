use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AllQueues {
    pub queues: Vec<Queues>,
}

#[derive(Serialize)]
pub struct Queues {
    pub name: String,
    pub total: usize,
}

#[derive(Serialize)]
pub struct Queue {
    pub queue: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct NewQueue {
    pub message: String,
}
