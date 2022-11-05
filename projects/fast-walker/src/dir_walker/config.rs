use crate::WalkItem;
use super::*;

impl<'i> IntoIterator for &'i WalkPlan {
    type Item = WalkItem;
    type IntoIter = WalkSearcher;

    fn into_iter(self) -> Self::IntoIter {
        let (task_in, task_out) = channel();
        for file in &self.check_list {
            task_in.send((file.clone(), 0)).unwrap();
        }

        WalkSearcher {
            check_list: task_in,
            task_out,
            follow_symlinks: self.follow_symlinks,
            depth_first: false,
            max_depth: self.max_depth,

            reject_directory: self.reject_directory,
        }
    }
}


impl WalkPlan {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut check_list = vec![path.as_ref().to_path_buf()];
        Self {
            check_list,
            follow_symlinks: false,
            depth_first: false,
            max_depth: usize::MAX,
            threads: 8,
            reject_directory: |_, _| false,
        }
    }
}
