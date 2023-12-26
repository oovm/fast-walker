use super::*;

use crate::WalkError;
use std::{collections::VecDeque, path::PathBuf};

impl<'i> IntoIterator for &'i WalkPlan {
    type Item = Result<WalkItem, WalkError>;
    type IntoIter = LinearWalker<'i>;

    fn into_iter(self) -> Self::IntoIter {
        LinearWalker { config: &self, tasks: self.check_list.clone().into_iter().collect(), results: vec![], found_files: 0 }
    }
}

pub struct LinearWalker<'i> {
    config: &'i WalkPlan,
    tasks: VecDeque<WalkItem>,
    results: Vec<Result<WalkItem, WalkError>>,
    found_files: usize,
}

impl<'i> LinearWalker<'i> {}

impl<'i> LinearWalker<'i> {
    fn pop(&mut self) -> Option<PathBuf> {
        if self.config.depth_first { self.tasks.pop_back() } else { self.tasks.pop_front() }
    }
    fn walk_through(&mut self, entry: PathBuf) {
        if entry.is_file() {
            self.found_files += 1;
            self.results.push(Ok(WalkItem::new(entry)));
            return;
        }
        'inner: for entry in entry.read_dir().unwrap() {
            match entry {
                Ok(child) => {
                    self.tasks.push_back(child.path());
                }
                Err(e) => {
                    // self.results.push(Err(WalkError::new(path.clone(), e)));
                    continue 'inner;
                }
            }
        }
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
                    self.walk_through(s);
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
