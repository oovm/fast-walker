use super::*;

impl WalkResultQueue {
    pub fn new(finish_condition: fn(&WalkItem) -> bool) -> Self {
        Self { state: Arc::new(Mutex::new(WalkResultState { results: Default::default(), finish_condition, stopped: false })) }
    }

    pub fn send_file(&self, path: PathBuf) {
        match self.state.lock() {
            Ok(mut o) => o.insert(WalkItem::File { path }),
            Err(e) => panic!("{:?}", e),
        }
    }
    pub fn send_directory(&self, path: PathBuf) {
        match self.state.lock() {
            Ok(mut o) => o.insert(WalkItem::Directory { path }),
            Err(e) => panic!("{:?}", e),
        }
    }
    pub fn send_error(&self, directory: PathBuf, error: std::io::Error) {
        match self.state.lock() {
            Ok(mut o) => o.insert(WalkItem::Error { directory, error }),
            Err(_) => {}
        }
    }
    // pub fn terminate(&self) {
    //     match self.state.lock() {
    //         Ok(mut o) => o.stopped = true,
    //         Err(_) => {}
    //     }
    // }
    pub fn receive(&self) -> Option<WalkItem> {
        match self.state.lock() {
            Ok(mut o) => o.results.pop_front(),
            Err(e) => panic!("{:?}", e),
        }
    }
}

impl WalkResultState {
    pub fn insert(&mut self, item: WalkItem) {
        if self.stopped {
            return;
        }
        if (self.finish_condition)(&item) {
            self.stopped = true;
        }
        else {
            self.results.push_back(item);
        }
    }
}
