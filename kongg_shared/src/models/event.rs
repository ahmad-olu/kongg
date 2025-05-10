use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EventType {
    Created,
    Renamed,
    Deleted,
    Moved,
    Copied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub e_type: EventType,
    pub path: String,
}

impl Event {
    pub fn new(e_type: EventType, path: String) -> Self {
        Self { e_type, path }
    }
}
