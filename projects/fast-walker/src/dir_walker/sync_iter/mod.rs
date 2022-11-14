use crate::WalkItem;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
mod result;
mod task;
use crate::WalkPlan;

#[derive(Clone)]
pub struct WalkTaskQueue {
    tasks: Arc<Mutex<VecDeque<(PathBuf, usize)>>>,
    depth_first: bool,
}

#[derive(Clone)]
pub struct WalkResultQueue {
    state: Arc<Mutex<WalkResultState>>,
}

pub struct WalkResultState {
    results: VecDeque<WalkItem>,
    stopped: bool,
}

pub struct WalkSearcher {
    result_queue: WalkResultQueue,
}

impl Iterator for WalkSearcher {
    type Item = WalkItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.result_queue.receive()
    }
}

impl<'i> IntoIterator for &'i WalkPlan {
    type Item = WalkItem;
    type IntoIter = WalkSearcher;

    fn into_iter(self) -> Self::IntoIter {
        let result = WalkResultQueue::new();
        let result_queue = result.clone();
        let tasks = WalkTaskQueue::new(self.depth_first);
        tasks.send_roots(&self.check_list);
        let reject_directory = self.reject_when;
        // let finish_condition = self.finish_when;
        let handler = std::thread::spawn(move || {
            while let Some((path, depth)) = tasks.receive() {
                if reject_directory(&path, depth) {
                    continue;
                }
                match std::fs::read_dir(&path) {
                    Ok(read_dir) => {
                        for item in read_dir {
                            match item {
                                Ok(dir_entry) => match dir_entry.file_type() {
                                    Ok(file_type) => {
                                        let path = dir_entry.path();
                                        match file_type.is_dir() {
                                            true => {
                                                tasks.send(&path, depth + 1);
                                                result.send_directory(path)
                                            }
                                            false => {
                                                result.send_file(path);
                                            }
                                        }
                                    }
                                    Err(e) => result.send_error(path.clone(), e),
                                },
                                Err(e) => result.send_error(path.clone(), e),
                            }
                        }
                    }
                    Err(e) => result.send_error(path, e),
                }
            }
        });
        handler.join().unwrap();
        WalkSearcher { result_queue }
    }
}
