use std::collections::VecDeque;
use std::fs::{DirEntry, read_dir};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender, sync_channel, TryRecvError};
use crate::WalkItem;

mod config;

pub struct WalkPlan {
    pub check_list: Vec<PathBuf>,
    pub follow_symlinks: bool,
    pub depth_first: bool,
    pub max_depth: usize,
    pub threads: usize,
    pub reject_directory: fn(&Path, usize) -> bool,
}

pub struct BreadthFirstWalker {
    check_list: Vec<(PathBuf, usize)>,
    follow_symlinks: bool,
    max_depth: usize,
    reject_directory: fn(&Path, usize) -> bool,
}

impl WalkItem {
    pub fn file<P: AsRef<Path>>(path: P) -> Self {
        Self::File {
            path: path.as_ref().to_path_buf(),
            depth: 0,
        }
    }
    pub fn directory<P: AsRef<Path>>(path: P) -> Self {
        Self::Directory {
            path: path.as_ref().to_path_buf(),
            depth: 0,
        }
    }
    pub fn error<P: AsRef<Path>>(path: P, error: std::io::Error) -> Self {
        Self::Error {
            directory: path.as_ref().to_path_buf(),
            error,
        }
    }
}

impl Iterator for BreadthFirstWalker {
    type Item = WalkItem;

    fn next(&mut self) -> Option<Self::Item> {
        let (task_in, task_out) = channel();
        let (ret_in, ret_out) = channel();
        for file in self.check_list {
            task_in.send(file).unwrap();
        }
        // Spawn a thread to traverse the directory
        std::thread::spawn(move || {
            while let Ok((path, depth)) = task_out.recv() {
                if depth > self.max_depth || (self.reject_directory)(&path, depth) {
                    continue;
                }

                match std::fs::read_dir(&path) {
                    Ok(entries) => {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                let path = entry.path();
                                if path.is_dir() {
                                    task_in.send((path.clone(), depth + 1)).unwrap();
                                    ret_in.send(WalkItem::directory(path)).unwrap();
                                } else if path.is_file() {
                                    ret_in.send(WalkItem::file(path)).unwrap();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        ret_in.send(WalkItem::error(path, e)).unwrap();
                    }
                }
            }
        });

// Receive the results from the thread
        match ret_out.recv() {
            Ok(item) => Some(item),
            Err(_) => None,
        }
    }
}


#[test]
fn test() {
    let project = "C:\\P4Root\\project\\OtherPlanet";
    for item in WalkPlan::new(project).into_iter().take(10) {
        println!("{:?}", item);
    }
}