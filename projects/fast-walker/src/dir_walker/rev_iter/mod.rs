use super::*;

impl WalkPlan {
    pub fn ancestors(&self) -> AncestorWalker {
        AncestorWalker {
            config: self,
            tasks: self.check_list.iter().map(|s| WalkItem::from(s.as_path())).collect(),
            results: self.check_list.iter().map(|s| WalkItem::from(s.as_path())).collect(),
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
            self.results.push(entry);
            return;
        }
        self.read_directory(entry);
    }
    fn read_directory(&mut self, entry: WalkItem) {
        if (self.config.ignore_when)(&entry) {
            return;
        }
        match entry.path.parent() {
            Some(dir) => {
                let parent = WalkItem::from(dir).with_depth(entry.depth - 1);
                self.results.push(parent.clone());
                self.tasks.push_back(parent);
            }
            None => {}
        }
    }
}

impl<'i> Iterator for AncestorWalker<'i> {
    type Item = WalkItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self.results.pop() {
            Some(s) => return Some(s),
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
