use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

use super::event::EventType;

#[derive(Debug, Serialize)]
pub struct File {
    event_type: EventType, // ? "created", "updated", or "deleted"
    path: String,          // ? Full file path (e.g., /home/user/docs/file.txt)
    previous_path: Option<String>,
    file_name: Option<String>, // ? File name (can be extracted from path)
    extension: Option<String>, // ? File extension (e.g., .txt, .png)
    file_size: Option<i64>,    // ? Size of file in bytes (only for created or updated)
    hash: Option<String>,      // ? checksum/hash of the file content (e.g., SHA256)
    previous_hash: Option<String>,
    file_id: Option<String>, // ? Platform-specific unique file ID
    parent_directory: Option<String>,
    permissions: Option<String>,
    timestamp: DateTime<Utc>, // ? When the event occurred (in UTC or local time)
}

impl File {
    pub fn new(event_type: EventType, path: String) -> Self {
        Self {
            event_type,
            path,
            previous_path: None,
            extension: None,
            file_id: None,
            file_name: None,
            file_size: None,
            hash: None,
            parent_directory: None,
            permissions: None,
            previous_hash: None,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FileResponse {
    pub id: RecordId,
    pub event_type: EventType,
    pub path: String,
    _previous_path: Option<String>,
    _file_name: Option<String>,
    _extension: Option<String>,
    _file_size: Option<i64>,
    _hash: Option<String>,
    _previous_hash: Option<String>,
    _file_id: Option<String>,
    _parent_directory: Option<String>,
    _permissions: Option<String>,
    pub timestamp: DateTime<Utc>,
}
