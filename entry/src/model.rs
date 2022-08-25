use thiserror::Error;

pub struct Entry {
    id: String,
    timestamp: u32,
    content: String,
    priority: u16,
}

pub enum Save {
    Created(String),
    Updated,
}