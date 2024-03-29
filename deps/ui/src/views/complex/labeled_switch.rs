use refs::Weak;
use ui_proc::view;
use vents::Event;

use crate::{view::ViewData, Anchor, Sub, ToLabel, ViewSetup};
mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

use crate::{Label, Switch};

#[view]
pub struct LabeledSwitch {
    label:  Sub<Label>,
    switch: Sub<Switch>,

    pub selected: Event<bool>,
}

impl LabeledSwitch {
    pub fn on(&self) -> bool {
        self.switch.on()
    }

    pub fn text(&self) -> &str {
        self.label.text()
    }

    pub fn set_text(&mut self, text: impl ToLabel) -> &mut Self {
        self.label.set_text(text);
        self
    }

    pub fn set_on(&mut self, on: bool) {
        self.switch.set_on(on);
    }
}

impl ViewSetup for LabeledSwitch {
    fn setup(self: Weak<Self>) {
        self.label.place().blt(0).relative(Anchor::Width, self, 0.5);
        self.switch
            .place()
            .size(80, 40)
            .center_y()
            .between_super(self.label, Anchor::Right);

        self.switch.selected.val(move |on| self.selected.trigger(on));
    }
}
