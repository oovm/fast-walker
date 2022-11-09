use super::*;

#[derive(Clone)]
pub struct WalkTaskQueue {
    tasks: Arc<Mutex<VecDeque<(PathBuf, usize)>>>,
    depth_first: bool,
}

#[derive(Clone)]
pub struct WalkResultQueue {
    results: Arc<Mutex<VecDeque<WalkItem>>>,
}

impl WalkTaskQueue {
    pub fn new(depth_first: bool) -> Self {
        Self { tasks: Arc::new(Mutex::default()), depth_first }
    }
    pub fn send_roots(&self, paths: &[PathBuf]) {
        match self.tasks.lock() {
            Ok(mut o) => {
                o.extend(paths.iter().map(|p| (p.to_path_buf(), 0)));
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
    pub fn send(&self, path: &Path, depth: usize) -> bool {
        let path = match path.canonicalize() {
            Ok(o) => o,
            Err(_) => return false,
        };
        match self.tasks.lock() {
            Ok(mut o) => o.push_front((path, depth)),
            Err(_) => return false,
        }
        true
    }
    pub fn receive(&self) -> Option<(PathBuf, usize)> {
        match self.tasks.lock() {
            Ok(mut o) => {
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

impl WalkResultQueue {
    pub fn new() -> Self {
        Self { results: Arc::new(Mutex::default()) }
    }

    pub fn send_file(&self, path: PathBuf) {
        match self.results.lock() {
            Ok(mut o) => {
                o.push_back(WalkItem::File { path });
            }
            Err(e) => {
                panic! { "{:?}", e }
            }
        }
    }
    pub fn send_directory(&self, path: PathBuf) {
        match self.results.lock() {
            Ok(mut o) => {
                o.push_back(WalkItem::Directory { path });
            }
            Err(e) => {
                panic! { "{:?}", e }
            }
        }
    }
    pub fn send_error(&self, directory: PathBuf, error: std::io::Error) {
        match self.results.lock() {
            Ok(mut o) => {
                o.push_back(WalkItem::Error { directory, error });
            }
            Err(_) => {}
        }
    }
    pub fn receive(&self) -> Option<WalkItem> {
        match self.results.lock() {
            Ok(mut o) => o.pop_front(),
            Err(e) => {
                panic! { "{:?}", e }
            }
        }
    }
}
