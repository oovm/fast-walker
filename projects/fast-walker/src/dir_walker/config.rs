use super::*;

impl WalkPlan {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            check_list: vec![path.as_ref().to_path_buf()],
            follow_symlinks: false,
            depth_first: false,
            threads: 8,
            reject_directory: |_, _| false,
        }
    }
    pub fn with_breadth_first(mut self) -> Self {
        self.depth_first = false;
        self
    }
    pub fn with_depth_first(mut self) -> Self {
        self.depth_first = true;
        self
    }
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }
    pub fn reject_if(mut self, f: fn(&Path, usize) -> bool) -> Self {
        self.reject_directory = f;
        self
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
        let reject_directory = self.reject_directory;
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
