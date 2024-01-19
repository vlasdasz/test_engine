use anyhow::Result;
use log::debug;
use test_engine::Screen;
use ui::{layout::Anchor, refs::Weak, view, SubView, ViewSetup};
use ui_views::{IntView, Label};

use crate::view_tests::record_touches;

#[view]
struct LabelTestView {
    label:          SubView<Label>,
    text_size_view: SubView<IntView>,
}

impl ViewSetup for LabelTestView {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("ßšėčыў");
        self.label.place.back().size(280, 280).center();

        self.text_size_view
            .place
            .size(50, 100)
            .center_y()
            .anchor(Anchor::Right, self.label, 10);
        self.text_size_view.set_value(32).set_step(5);

        self.text_size_view.on_change.val(move |size| {
            self.label.set_text_size(size);
        });
    }
}

pub async fn test_label() -> Result<()> {
    Screen::set_test_view::<LabelTestView>(400, 400).await;

    record_touches().await;

    debug!("Int view test: OK");

    Ok(())
}