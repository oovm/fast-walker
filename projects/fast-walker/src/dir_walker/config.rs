use super::*;
use crate::WalkItem;

impl<'i> IntoIterator for &'i WalkPlan {
    type Item = WalkItem;
    type IntoIter = WalkSearcher;

    fn into_iter(self) -> Self::IntoIter {
        let queue = WalkTaskQueue::new(self.depth_first);
        queue.send_roots(&self.check_list);
        WalkSearcher {
            task_queue: queue,
            follow_symlinks: self.follow_symlinks,
            max_depth: self.max_depth,
            reject_directory: self.reject_directory,
        }
    }
}

impl WalkPlan {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            check_list: vec![path.as_ref().to_path_buf()],
            follow_symlinks: false,
            depth_first: false,
            max_depth: usize::MAX,
            threads: 8,
            reject_directory: |_, _| false,
        }
    }
}
