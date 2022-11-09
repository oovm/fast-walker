use crate::{WalkItem, WalkResultQueue, WalkTaskQueue};
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::{mpsc::channel, Arc, Mutex},
};

mod config;

pub mod queue;

pub struct WalkPlan {
    pub check_list: Vec<PathBuf>,
    pub follow_symlinks: bool,
    pub depth_first: bool,
    pub max_depth: usize,
    pub threads: usize,
    pub reject_directory: fn(&Path, usize) -> bool,
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

#[test]
fn test() {
    let project = "C:\\P4Root\\project\\OtherPlanet";
    for item in WalkPlan::new(project).into_iter().take(10) {
        println!("{:?}", item);
    }
}
