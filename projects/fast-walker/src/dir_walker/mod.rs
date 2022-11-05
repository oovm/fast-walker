use std::collections::VecDeque;
use std::fs::{DirEntry, read_dir};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender, sync_channel};


#[derive(Debug)]
pub enum WalkItem {
    File {
        path: PathBuf,
        depth: usize,
    },
    Directory {
        path: PathBuf,
        depth: usize,
    },
    Error {
        error: std::io::Error
    },
}

pub struct FastWalkerIterator {
    directory_list: VecDeque<PathBuf>,
    follow_symlinks: bool,
    depth_first: bool,
    max_depth: usize,
    max_open: usize,
    sender: Sender<WalkItem>,
    receiver: Receiver<WalkItem>,
    /// path: The path of the directory
    /// depth: The depth of the directory
    /// return: true if the directory should be skipped
    reject_directory: fn(&Path, usize) -> bool,
}

impl FastWalkerIterator {
    fn next_search(&mut self) -> Option<PathBuf> {
        match self.depth_first {
            true => { self.directory_list.pop_back() }
            false => { self.directory_list.pop_front() }
        }
    }
}


impl Iterator for FastWalkerIterator {
    type Item = WalkItem;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.next_search() {
            let path = item.clone();
            let depth = match item.strip_prefix(&self.directory_list[0]) {
                Ok(p) => self.directory_list[0].components().count() + p.components().count() - 1,
                Err(_) => 0,
            };
            if depth > self.max_depth {
                continue;
            }
            if let Ok(metadata) = std::fs::symlink_metadata(&path) {
                if metadata.is_dir() {
                    if (self.reject_directory)(&path, depth) {
                        continue;
                    }
                    if self.follow_symlinks || !metadata.file_type().is_symlink() {
                        if self.max_open > 0 {
                            while self.directory_list.len() >= self.max_open {
                                match self.receiver.recv() {
                                    Ok(WalkItem::Directory { .. }) => {}
                                    Ok(_) => {}
                                    Err(_) => break,
                                }
                            }
                        }
                        self.sender.send(WalkItem::Directory { path: path.clone(), depth }).unwrap();
                        match std::fs::read_dir(&path) {
                            Ok(dir) => {
                                let mut entries: Vec<_> = dir.filter_map(|e| e.ok()).collect();
                                if !self.depth_first {
                                    entries.reverse();
                                }
                                for entry in entries {
                                    self.directory_list.push_front(entry.path());
                                }
                            }
                            Err(e) => {
                                self.sender.send(WalkItem::Error { error: e }).unwrap();
                            }
                        }
                    }
                } else {
                    self.sender.send(WalkItem::File { path, depth }).unwrap();
                }
            } else {
                self.sender.send(WalkItem::Error { error: std::io::Error::new(std::io::ErrorKind::Other, format!("Could not get metadata for {:?}", path)) }).unwrap();
            }
        }
        None
    }
}


impl FastWalker {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut check_list = vec![path.as_ref().to_path_buf()];
        Self {
            check_list,
            follow_symlinks: false,
            max_depth: None,
            max_open: None,
            threads: 8,
            reject_directory: |_, _| false,
        }
    }
}

#[test]
fn test() {
    let project = "C:\\P4Root\\project\\OtherPlanet";
    for item in FastWalker::new(project).into_iter().take(10) {
        println!("{:?}", item);
    }
}