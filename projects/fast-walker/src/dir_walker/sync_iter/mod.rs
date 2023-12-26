use super::*;

use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex, MutexGuard,
    },
    thread,
    thread::JoinHandle,
};

#[derive(Clone)]
pub struct ThreadWalker {
    config: Arc<WalkPlan>,
    tasks: Arc<Mutex<VecDeque<PathBuf>>>,
    result_send: Arc<Mutex<Sender<WalkItem>>>,
    result_read: Arc<Mutex<Receiver<WalkItem>>>,
    found_files: Arc<Mutex<usize>>,
}

impl ThreadWalker {
    pub fn new(plan: &WalkPlan) -> Self {
        let walker = Self::initialize(plan);
        walker.tasks.lock().unwrap().extend(plan.check_list.iter().cloned());
        for _ in 0..plan.threads {
            let _ = walker.clone().spawn().join().unwrap();
        }
        walker
    }
    fn initialize(plan: &WalkPlan) -> Self {
        let (result_send, result_read) = channel();
        Self {
            config: Arc::new(plan.clone()),
            tasks: Arc::new(Mutex::new(VecDeque::default())),
            result_send: Arc::new(Mutex::new(result_send)),
            result_read: Arc::new(Mutex::new(result_read)),
            found_files: Arc::new(Mutex::new(0)),
        }
    }
}

impl ThreadWalker {
    fn spawn(self) -> JoinHandle<()> {
        thread::spawn(move || {
            'outer: loop {
                let (mut tasks, path) = match self.pop() {
                    Some(o) => o,
                    None => break 'outer,
                };
                println!("Catch: {:?}", path);
                if path.is_file() {
                    match self.result_send.try_lock().unwrap().send(WalkItem::File { path: path.clone() }) {
                        Ok(_) => {}
                        Err(e) => panic!("{:?}", e),
                    };
                    continue 'outer;
                }
                'inner: for entry in path.read_dir().unwrap() {
                    match entry {
                        Ok(child) => {
                            tasks.push_back(child.path());
                        }
                        Err(e) => {
                            self.result_send
                                .try_lock()
                                .unwrap()
                                .send(WalkItem::Error { path: path.clone(), error: e })
                                .unwrap();
                            continue 'inner;
                        }
                    }
                }
            }
        })
    }

    fn pop(&self) -> Option<(MutexGuard<VecDeque<PathBuf>>, PathBuf)> {
        match self.tasks.try_lock() {
            Ok(mut o) => {
                let path = if self.config.depth_first { o.pop_back()? } else { o.pop_front()? };
                Some((o, path))
            }
            Err(_) => None,
        }
    }
}

impl<'i> Iterator for ThreadWalker {
    type Item = WalkItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self.result_read.try_lock().unwrap().try_recv() {
            Ok(item) => Some(item),
            Err(e) => {
                println!("{:?}", e);

                None
            }
        }
    }
}

#[test]
fn run() {
    let plan = WalkPlan {
        check_list: vec![
            PathBuf::from(r#"C:\Users\Dell\CLionProjects\fast-walker"#),
            // PathBuf::from(r#"C:\Users\Dell\CLionProjects\fast-walker\projects\"#),
        ],
        follow_symlinks: true,
        depth_first: false,
        capacity: 4,
        threads: 4,
        reject_when: |_, _| false,
        ignore_when: |_| false,
        finish_when: |_| false,
    };

    let walker = ThreadWalker::new(&plan);

    for item in walker.take(10) {
        match item {
            WalkItem::File { path } => {
                println!("File: {:?}", path);
            }
            WalkItem::Directory { path } => {
                println!("Directory: {:?}", path);
            }
            WalkItem::Error { path, error } => {
                println!("Error: {:?}, {:?}", path, error);
            }
        }
    }
}
