use super::*;
use crate::{dir_walker::sync_iter::LinearWalker, WalkError};

impl WalkPlan {
    pub fn reversed(self) -> AncestorWalker {
        AncestorWalker {
            config: self,
            tasks: self.check_list.iter().map(|s| WalkItem::from(s.as_path())).collect(),
            results: vec![],
        }
    }
}

pub struct AncestorWalker<'i> {
    pub config: &'i WalkPlan,
    pub tasks: VecDeque<WalkItem>,
    pub results: Vec<WalkItem>,
}

impl<'i> AncestorWalker<'i> {
    fn pop(&mut self) -> Option<WalkItem> {
        if self.config.depth_first { self.tasks.pop_back() } else { self.tasks.pop_front() }
    }
    fn read_item(&mut self, entry: WalkItem) {
        if (self.config.finish_when)(&entry) {
            self.tasks.clear();
            self.results.push(Ok(entry));
            return;
        }
        self.read_directory(entry);
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
}

impl Iterator for AncestorWalker {
    type Item = WalkItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self.results.pop() {
            Some(s) => {
                return Some(s);
            }
            None => {
                if self.tasks.is_empty() {
                    return None;
                }
                let entry = self.tasks.pop_front().unwrap();
                if (self.config.finish_when)(&entry) {
                    self.tasks.clear();
                    return Some(entry);
                }
                if entry.path.is_symlink() {
                    self.read_link(entry);
                    return self.next();
                }
                if entry.is_directory() {
                    self.read_directory(entry);
                    return self.next();
                }
                self.read_file(entry);
                return self.next();
            }
        }
    }
}
