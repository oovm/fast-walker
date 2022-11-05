use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use crate::dir_walker::{FastWalkerIterator, WalkItem};




impl<'i> IntoIterator for &'i FastWalker {
    type Item = WalkItem;
    type IntoIter = FastWalkerIterator;

    fn into_iter(self) -> Self::IntoIter {
        let (sender, receiver) = channel();
        FastWalkerIterator {
            directory_list: self.check_list.iter().cloned().collect(),
            follow_symlinks: self.follow_symlinks,
            depth_first: self.depth_first,
            max_depth: self.max_depth.unwrap_or(usize::MAX),
            max_open: self.max_open.unwrap_or(usize::MAX),
            sender,
            receiver,
            reject_directory: self.reject_directory,
        }
    }
}
