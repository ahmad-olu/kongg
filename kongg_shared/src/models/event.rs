use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EventType {
    Created,
    Renamed,
    Deleted,
    Moved,
    Copied,
}

impl ToString for EventType {
    fn to_string(&self) -> String {
        match self {
            EventType::Created => String::from("Created"),
            EventType::Renamed => String::from("Renamed"),
            EventType::Deleted => String::from("Deleted"),
            EventType::Moved => String::from("Moved"),
            EventType::Copied => String::from("Copied"),
        }
    }
}

// ! conflict with ToString trait
// impl fmt::Display for EventType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }

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
