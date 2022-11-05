use std::collections::VecDeque;
use std::fs::{DirEntry, read_dir};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender, sync_channel, TryRecvError};
use std::sync::{Arc, Mutex};
use crate::WalkItem;

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
    check_list: VecDeque<(PathBuf, usize)>,
    follow_symlinks: bool,

    max_depth: usize,
    reject_directory: fn(&Path, usize) -> bool,
}




impl WalkSearcher {
    fn search_next(&mut self) -> Option<(PathBuf, usize)> {
        if self.depth_first {
            self.check_list.pop_front()
        } else {
            self.check_list.pop_back()
        }
    }
}


impl Iterator for WalkSearcher {
    type Item = WalkItem;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((dir, depth)) = self.search_next() {

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