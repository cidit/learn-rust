use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub timestamp: u32,
    pub content: String,
    pub priority: u16,
}

#[derive(Serialize, Deserialize)]
pub enum Save {
    Created(String),
    Updated,
}
