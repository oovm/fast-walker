use crate::WalkItem;
use super::*;

impl<'i> IntoIterator for &'i WalkPlan {
    type Item = WalkItem;
    type IntoIter = BreadthFirstWalker;

    fn into_iter(self) -> Self::IntoIter {
        BreadthFirstWalker {
            check_list: self.check_list.iter().map(|path| (path.clone(), 0)).collect(),
            follow_symlinks: self.follow_symlinks,
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
