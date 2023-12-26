use super::*;

impl WalkPlan {
    pub fn reversed(self) -> AncestorWalker {
        AncestorWalker {
            config: self,
            tasks: self.check_list.iter().map(|s| WalkItem::from(s.as_path())).collect(),
            results: vec![],
        }
    }
}

pub struct AncestorWalker {
    pub config: WalkPlan,
    pub tasks: VecDeque<WalkItem>,
    pub results: Vec<WalkItem>,
}

impl Iterator for AncestorWalker {
    type Item = WalkItem;

    fn next(&mut self) -> Option<Self::Item> {
        match self.results.pop() {
            None => {}
            Some(_) => {}
        }
    }
}
