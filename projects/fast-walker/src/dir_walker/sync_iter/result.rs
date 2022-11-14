use super::*;

impl WalkResultQueue {
    pub fn new() -> Self {
        Self { results: Arc::new(Mutex::default()), stopped: Arc::new(Mutex::new(false)) }
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
