use std::{
    path::{Path, PathBuf},
    sync::{Arc, LockResult, Mutex},
};

#[derive(Clone)]
pub struct WalkTaskQueue {
    tasks: Arc<Mutex<(PathBuf, usize)>>,
    depth_first: bool,
}

impl WalkTaskQueue {
    pub fn new(depth_first: bool) -> Self {
        Self { tasks: Arc::new(Mutex::default()), depth_first }
    }
    pub fn send(&self, path: &Path, depth: usize) -> bool {
        match self.tasks.lock() {
            Ok(o) => {
                o.push((path.to_path_buf(), depth));
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
        true
    }
    pub fn next(&self) -> Option<(PathBuf, usize)> {
        match self.tasks.lock() {
            Ok(o) => {
                if self.depth_first {
                    o.pop_front()
                }
                else {
                    o.pop_back()
                }
            }
            Err(e) => {
                panic! { "{:?}", e }
            }
        }
    }
}
