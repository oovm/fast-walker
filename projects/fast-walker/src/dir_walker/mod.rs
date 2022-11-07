use crate::{dir_walker::queue::WalkTaskQueue, WalkItem};
use std::{
    collections::VecDeque,
    fs::{read_dir, DirEntry},
    path::{Path, PathBuf},
    sync::{
        mpsc::{channel, sync_channel, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
};

mod config;

mod queue;

pub struct WalkPlan {
    pub check_list: Vec<PathBuf>,
    pub follow_symlinks: bool,
    pub depth_first: bool,
    pub max_depth: usize,
    pub threads: usize,
    pub reject_directory: fn(&Path, usize) -> bool,
}

pub struct WalkSearcher {
    task_queue: WalkTaskQueue,
    follow_symlinks: bool,
    max_depth: usize,
    reject_directory: fn(&Path, usize) -> bool,
}

impl Iterator for WalkSearcher {
    type Item = WalkItem;

    fn next(&mut self) -> Option<Self::Item> {
        let (tx, rx) = channel();
        let tasks = self.task_queue.clone();
        std::thread::spawn(| | {
            while let Some((path, depth)) = self.task_queue.receive() {
                if depth > self.max_depth || (self.reject_directory)(&path, depth) {
                    continue;
                }
                match std::fs::read_dir(&path) {
                    Ok(read_dir) => {
                        for item in read_dir {
                            match item {
                                Ok(dir_entry) => {
                                    match dir_entry.file_type() {
                                        Ok(file_type) => {
                                            let path =  dir_entry.path();
                                            match file_type.is_dir() {
                                                true => {
                                                    tasks.send(&path, depth + 1)
                                                }
                                                false => {
                                                    tx.send(WalkItem::file(path, depth + 1)).unwrap();
                                                }
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                    
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    Err(e) => {
                        tx.send(WalkItem::error(path, e)).unwrap();
                    }
                }
            }
            true
    
        });
        for out in rx {
            return Some(out);
        }
        None
    }
}

#[test]
fn test() {
    let project = "C:\\P4Root\\project\\OtherPlanet";
    for item in WalkPlan::new(project).into_iter().take(10) {
        println!("{:?}", item);
    }
}
