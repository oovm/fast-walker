use super::*;

impl PathDepth {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self { path: path.as_ref().to_path_buf(), depth: 0 }
    }
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }
}

impl WalkTaskQueue {
    pub fn new(depth_first: bool, capacity: usize) -> Self {
        Self { tasks: Arc::new(Mutex::default()), max_size: capacity, depth_first }
    }
    pub fn send_roots<P>(&self, paths: &[P])
    where
        P: AsRef<Path>,
    {
        match self.tasks.lock() {
            Ok(mut o) => {
                o.extend(paths.iter().map(|p| PathDepth::new(p)));
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
            Ok(mut o) => o.push_front(PathDepth::new(path).with_depth(depth)),
            Err(_) => return false,
        }
        true
    }
    pub fn receive(&self) -> Option<PathDepth> {
        match self.tasks.lock() {
            Ok(mut o) => {
                if self.depth_first {
                    o.pop_front()
                }
                else {
                    o.pop_back()
                }
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
