use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use surrealdb::RecordId;

use super::event::EventType;

#[derive(Debug, Serialize)]
pub struct File {
    event_type: EventType, // ? "created", "updated", or "deleted"
    path: String,          // ? Full file path (e.g., /home/user/docs/file.txt)
    previous_path: Option<String>,
    file_name: Option<String>, // ? File name (can be extracted from path)
    extension: Option<String>, // ? File extension (e.g., .txt, .png)
    file_size: Option<u64>,    // ? Size of file in bytes (only for created or updated)
    hash: Option<String>,      // ? checksum/hash of the file content (e.g., SHA256)
    previous_hash: Option<String>,
    file_id: Option<String>, // ? Platform-specific unique file ID
    parent_directory: Option<String>,
    permissions: Option<String>,
    is_read_only: bool,
    timestamp: DateTime<Utc>, // ? When the event occurred (in UTC or local time)
}

impl File {
    pub fn new(event_type: EventType, path: String) -> Self {
        let path = path.clone();
        // let extension = path
        //     .clone()
        //     .split(".")
        //     .collect::<Vec<&str>>()
        //     .last()
        //     .map(|v| v.to_string());

        let normalized_path = Path::new(&path).canonicalize().unwrap();
        let metadata = fs::metadata(&normalized_path).unwrap();

        let parent_directory = normalized_path.parent().map(|p| p.display().to_string());
        let file_name = normalized_path
            .file_name()
            .map(|f| f.to_string_lossy().to_string());
        let file_size = Some(metadata.len());

        let extension = normalized_path
            .extension()
            .map(|e| e.to_string_lossy().to_string());

        let is_read_only = metadata.permissions().readonly();

        Self {
            event_type,
            path,
            previous_path: None,
            extension,
            file_id: None,
            file_name,
            file_size,
            hash: None,
            parent_directory,
            permissions: None,
            previous_hash: None,
            is_read_only,
            timestamp: Utc::now(),
        }
    }

    pub fn add_previous_path(mut self, path: String) -> Self {
        self.previous_path = Some(path);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct FileResponse {
    pub id: RecordId,
    pub event_type: EventType,
    pub path: String,
    pub previous_path: Option<String>,
    pub file_name: Option<String>,
    pub extension: Option<String>,
    pub file_size: Option<u64>,
    _hash: Option<String>,
    _previous_hash: Option<String>,
    _file_id: Option<String>,
    pub parent_directory: Option<String>,
    pub permissions: Option<String>,
    pub is_read_only: bool,
    pub timestamp: DateTime<Utc>,
}
