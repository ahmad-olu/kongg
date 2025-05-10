# 📁 Kongg

A lightweight Rust application that monitors your file system for **created**, **updated**, and **deleted** files and logs all events to a local database.

# 🎯 Motivation
I built Kongg out of a personal need: I often download movies, videos, and media files, then delete them later to free up space — only to realize months later that I want to watch them again but can't remember what they were called.

This tool helps solve that problem by keeping a simple record of every file that was created, modified, or deleted on my system — especially in directories like Downloads, Videos, or external drives. Now, even if I delete a movie or video, I can go back and find the name later, and re-download or retrieve it.

Kongg is like a memory log for your file system, helping you remember what was once there, even after it's gone.

## 🚀 Features

- Tracks real-time file system events:
  - File creation
  - File modification
  - File deletion
- Stores event details in a persistent database
<!-- - Hashes file contents for change verification -->
- Cross-platform support (Linux, macOS, Windows)
<!-- - Extensible for history tracking or audit systems -->

## 🧠 How It Works

`Kongg` uses the [`notify`](https://crates.io/crates/notify) crate to watch directories and capture file system events. When a file is created, modified, or deleted, the application:

1. Captures metadata about the file
2. (If applicable) Hashes the content to detect changes
3. Saves the event and details into a SurrealDb

## 📦 Database Schema

| Field           | Type      | Description                             |
|----------------|-----------|-----------------------------------------|
| `id`           | UUID      | Unique event ID                         |
| `event_type`   | TEXT      | "created", "updated", or "deleted"      |
| `path`         | TEXT      | Full path to the file                   |
| `filename`     | TEXT      | File name only                          |
| `extension`    | TEXT      | File extension                          |
| `timestamp`    | DATETIME  | Event occurrence time (UTC)             |
| `file_size`    | INTEGER   | File size in bytes (nullable for delete)|
| `hash`         | TEXT      | SHA256 hash of file content             |
| `previous_hash`| TEXT      | Previous hash (for updates)             |
| `is_directory` | BOOLEAN   | Whether the path is a directory         |

## ⚙️ Installation

## 📈 Use Cases
- File auditing
- Security and intrusion detection
- Backup and sync tools
- Developer utilities

## 🧩 Future Plans
- Web dashboard for event browsing
- JSON export or cloud sync
- Event filters (e.g., only .txt files)

## 🤝 Contributing
Pull requests are welcome! For major changes, open an issue first to discuss what you would like to change.
