extern crate notify;

use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::fs;
use std::{path::PathBuf, sync::mpsc::channel};

const DOWNLOAD_PATH: &str = "/home/neo/Downloads";
const SONG_FOLDER_PATH: &str = "/home/neo/Games/osu/drive_c/osu/Songs";

fn main() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering raw events.
    // The notification back-end is selected based on the platform.
    let mut watcher = raw_watcher(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(DOWNLOAD_PATH, RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie: _,
                ..
            }) => {
                if let notify::op::CHMOD = op {
                    if is_osz_file(&path) {
                        move_osz_file(&path);
                    }
                }
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn is_osz_file(file: &PathBuf) -> bool {
    match file.extension() {
        Some(ext) => {
            if ext == "osz" {
                true
            } else {
                false
            }
        }
        None => false,
    }
}

fn move_osz_file(file: &PathBuf) {
    let filename = file.file_name().unwrap().to_str().unwrap();
    // concat strings into one path
    let dest_path: PathBuf = [SONG_FOLDER_PATH, filename].iter().collect();
    // move file
    let res = fs::rename(file, dest_path);
    match res {
        Ok(_) => {
            println!("Moved file")
        }
        Err(_) => {
            println!("Move failed")
        }
    }
}
