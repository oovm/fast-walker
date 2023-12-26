use super::*;

impl<'i> IntoIterator for &'i WalkPlan {
    type Item = Result<WalkItem, WalkError>;
    type IntoIter = LinearWalker<'i>;

    fn into_iter(self) -> Self::IntoIter {
        LinearWalker {
            config: &self,
            tasks: self.check_list.iter().map(|s| WalkItem::from(s.as_path())).collect(),
            results: vec![],
            found_files: 0,
        }
    }
}

pub struct LinearWalker<'i> {
    config: &'i WalkPlan,
    tasks: VecDeque<WalkItem>,
    results: Vec<Result<WalkItem, WalkError>>,
    found_files: usize,
}

impl<'i> LinearWalker<'i> {
    fn pop(&mut self) -> Option<WalkItem> {
        if self.config.depth_first { self.tasks.pop_back() } else { self.tasks.pop_front() }
    }
    fn read_item(&mut self, entry: WalkItem) {
        if (self.config.finish_when)(&entry) {
            self.tasks.clear();
            self.results.push(Ok(entry));
            return;
        }
        if entry.path.is_symlink() {
            self.read_link(entry);
            return;
        }
        if entry.is_directory() {
            self.read_directory(entry);
            return;
        }
        self.read_file(entry)
    }
    fn read_link(&mut self, entry: WalkItem) {
        if self.config.follow_symlinks {
            match entry.read_link() {
                Ok(o) => {
                    self.tasks.push_back(WalkItem::from(o).with_depth(entry.depth + 1));
                }
                Err(e) => {
                    self.results.push(Err(WalkError::io_error(entry.path, e)));
                }
            }
        }
    }
    fn read_directory(&mut self, entry: WalkItem) {
        if (self.config.ignore_when)(&entry) {
            return;
        }
        match entry.read_directory() {
            Ok(dir) => {
                for result in dir {
                    match result {
                        Ok(child) => {
                            self.tasks.push_back(WalkItem::from(child).with_depth(entry.depth + 1));
                        }
                        Err(e) => {
                            self.results.push(Err(WalkError::io_error(entry.path.clone(), e)));
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                self.results.push(Err(WalkError::io_error(entry.path, e)));
            }
        }
    }
    fn read_file(&mut self, entry: WalkItem) {
        debug_assert!(entry.is_file());
        if (self.config.reject_when)(&entry) {
            return;
        }
        self.found_files += 1;
        self.results.push(Ok(entry));
    }
}

impl<'i> Iterator for LinearWalker<'i> {
    type Item = Result<WalkItem, WalkError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.results.pop() {
            Some(s) => {
                return Some(s);
            }
            None => match self.pop() {
                Some(s) => {
                    self.read_item(s);
                    self.next()
                }
                None => None,
            },
        }
    }
}

#[test]
fn run() {
    let plan = WalkPlan {
        check_list: vec![
            PathBuf::from(r#"C:\Users\Dell\CLionProjects\fast-walker"#),
            PathBuf::from(r#"C:\Users\Dell\CLionProjects\faster-pest"#),
        ],
        follow_symlinks: true,
        depth_first: true,
        capacity: 4,
        threads: 4,
        reject_when: |_| false,
        ignore_when: |_| false,
        finish_when: |_| false,
    };

    for item in plan.into_iter().take(10) {
        match item {
            Ok(o) => {
                println!("File: {:?}", o);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
