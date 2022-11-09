use super::*;
use crate::WalkItem;

impl<'i> IntoIterator for &'i WalkPlan {
    type Item = WalkItem;
    type IntoIter = WalkSearcher;

    fn into_iter(self) -> Self::IntoIter {
        let result = WalkResultQueue::new();
        let tasks = WalkTaskQueue::new(self.depth_first);
        tasks.send_roots(&self.check_list);
        let max_depth = self.max_depth;
        let reject_directory = self.reject_directory;
        std::thread::spawn(move || {
            while let Some((path, depth)) = tasks.receive() {
                if depth > max_depth || reject_directory(&path, depth) {
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
            true
        });
        WalkSearcher { result_queue: result }
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
