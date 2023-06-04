use refs::Weak;

use crate::View;

pub(crate) struct TouchLayer {
    root:    Weak<dyn View>,
    touches: Vec<Weak<dyn View>>,
}

impl TouchLayer {
    pub(crate) fn add(&mut self, view: Weak<dyn View>) {
        self.touches.retain(|a| !a.freed());
        self.touches.push(view);
    }

    pub(crate) fn remove(&mut self, view: Weak<dyn View>) {
        self.touches.retain(|a| a.addr() != view.addr());
    }

    pub(crate) fn views(&self) -> Vec<Weak<dyn View>> {
        self.touches.clone()
    }

    pub(crate) fn root_addr(&self) -> usize {
        self.root.addr()
    }

    pub(crate) fn root_name(&self) -> String {
        self.root.label.clone()
    }
}

impl From<Weak<dyn View>> for TouchLayer {
    fn from(root: Weak<dyn View>) -> Self {
        Self {
            root,
            touches: vec![],
        }
    }
}
