// everything related to scanning a folder for mvoies , tv series , etc .

use std::fs;
use std::path::{Path, PathBuf};

// video file extensions (99.99 percent)
const VIDEO_EXTENSIONS: [&str; 5] = ["mkv", "mp4", "avi", "mov" , "webm"];


pub fn walk_dir(folder: &Path) -> Vec<PathBuf> {
    let mut found_files = Vec::new();

    if fs::read_dir(folder).is_err() {
        return found_files; //some error , return empty vec
    }
    let entries = fs::read_dir(folder).unwrap();

    for entry in entries {
        if entry.is_err() {
            continue; // bad entry, skip it
        }
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            // its a folder, go inside too
            let mut inner_files = walk_dir(&path);
            found_files.append(&mut inner_files);
        } else if is_video_file(&path) {
            found_files.push(path);
        }
    }

    found_files
}

// checks if a file path ends with a video extension we care about
fn is_video_file(path: &Path) -> bool {
    let extension = path.extension();
    if extension.is_none() {
        return false; // no extension, skip
    }
    let extension = extension.unwrap();

    let extension = extension.to_str();
    if extension.is_none() {
        return false;
    }

    let extension = extension.unwrap();

    for allowed in VIDEO_EXTENSIONS.iter() {
        if extension.eq_ignore_ascii_case(allowed) {
            return true;
        }
    }

    false
}
