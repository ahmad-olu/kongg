use futures::{
    SinkExt, StreamExt,
    channel::mpsc::{Receiver, channel},
};
use globset::{Glob, GlobSet, GlobSetBuilder};
use kongg_shared::{
    helpers::{crud::create_file, surreal_init::init},
    models::{
        event::{Event as KEvent, EventType},
        file::File,
    },
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tokio::task;
use tokio::time::{Duration, sleep};

//cargo run .

//const IGNORED_PATHS: &[&str] = &[".git"];

fn build_ignore_list() -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    let patterns = vec![
        "**/.git/**",         // Ignore all .git directories
        "**/node_modules/**", // Ignore node_modules anywhere
        "**/target/**",       // Ignore Rust target directory
        "**/*.log",           // Ignore all .log files
        "**/.gitignore",
    ];

    for pattern in patterns {
        builder.add(Glob::new(pattern).expect("Invalid glob pattern"));
    }

    builder.build().expect("Failed to build globset")
}

//Todo* //Note: copy and paste returns a `Create event` while cut/move and pase return a `Rename event` with the 1st = old location and 2nd = new location

#[tokio::main]
async fn main() {
    let e = KEvent::new(EventType::Created, "aab".to_string());
    println!("Hello, world ==> !, {:?}", e);

    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);

    futures::executor::block_on(async {
        if let Err(e) = async_watch(path).await {
            println!("error: {:?}", e)
        }
    });
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let db = init().await.unwrap();
    let (mut watcher, mut rx) = async_watcher()?;
    let ignore_list = build_ignore_list(); // Load ignore patterns

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    let recent_events = Arc::new(Mutex::new(HashMap::<PathBuf, bool>::new()));

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                let mut events = recent_events.lock().unwrap();
                if event.paths.iter().any(|p| ignore_list.is_match(p)) {
                    continue; // Skip ignored patterns
                }

                match event.kind {
                    notify::EventKind::Remove(remove_kind) => {
                        // create_file(
                        //     &db,
                        //     File::new(
                        //         EventType::Deleted,
                        //         event.paths[0].to_string_lossy().to_string(),
                        //     ),
                        // )
                        // .await;
                        println!("removed ${:?}", remove_kind);
                    }
                    notify::EventKind::Create(create_kind) => match create_kind {
                        notify::event::CreateKind::File | notify::event::CreateKind::Folder => {
                            create_file(
                                &db,
                                File::new(
                                    EventType::Created,
                                    event.paths[0].to_string_lossy().to_string(),
                                ),
                            )
                            .await;
                            println!("created =>: {:?}", event.paths)
                        }
                        _ => (),
                    },
                    notify::EventKind::Modify(modify_kind) => {
                        match modify_kind {
                            notify::event::ModifyKind::Name(_rename_mode) => {
                                let from = event.paths.get(0);
                                let to = event.paths.get(1);
                                match (from, to) {
                                    (Some(old_path), None) => {
                                        println!(
                                            "-------------------- 1 ======={:?}/ {:?}",
                                            modify_kind, old_path
                                        );
                                        let old_path_clone = old_path.clone();
                                        events.insert(old_path_clone.clone(), true);

                                        // 🕒 Wait before confirming deletion
                                        let events_clone = Arc::clone(&recent_events);
                                        task::spawn({
                                            let db = db.clone();
                                            async move {
                                                sleep(Duration::from_millis(100)).await;
                                                let should_create_file = {
                                                    let mut events = events_clone.lock().unwrap();
                                                    events.remove(&old_path_clone).is_some()
                                                };
                                                if should_create_file {
                                                    create_file(
                                                        &db,
                                                        File::new(
                                                            EventType::Deleted,
                                                            event.paths[0]
                                                                .to_string_lossy()
                                                                .to_string(),
                                                        ),
                                                    )
                                                    .await;
                                                    // println!(
                                                    //     "✅ File Deleted: {:?}",
                                                    //     old_path_clone
                                                    // );
                                                }
                                            }
                                        });
                                    }
                                    (Some(old_path), Some(new_path)) => {
                                        // 🔄 Confirm Rename (Remove from deletion tracking)

                                        let old_file_name = old_path
                                            .to_string_lossy()
                                            .to_string()
                                            .rsplit(['/', '\\'])
                                            .next()
                                            .map(|s| s.to_string());

                                        let new_file_name = Path::new(&new_path)
                                            .canonicalize()
                                            .ok()
                                            .and_then(|p| {
                                                p.file_name()
                                                    .map(|f| f.to_string_lossy().to_string())
                                            });

                                        if old_file_name == new_file_name {
                                            create_file(
                                                &db,
                                                File::new(
                                                    EventType::Moved,
                                                    new_path.to_string_lossy().to_string(),
                                                )
                                                .add_previous_path(
                                                    old_path.to_string_lossy().to_string(),
                                                ),
                                            )
                                            .await;
                                        } else {
                                            create_file(
                                                &db,
                                                File::new(
                                                    EventType::Renamed,
                                                    new_path.to_string_lossy().to_string(),
                                                )
                                                .add_previous_path(
                                                    old_path.to_string_lossy().to_string(),
                                                ),
                                            )
                                            .await;
                                        }
                                        events.remove(old_path);
                                        events.remove(new_path);

                                        println!(
                                            "📂 File Renamed: {:?} -> {:?}",
                                            old_path, new_path
                                        );
                                    }
                                    _ => {}
                                }
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
