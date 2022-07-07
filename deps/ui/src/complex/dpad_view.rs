use gl_image::Image;
use gm::flat::Direction;
use rtools::{data_manager::Handle, Event, Rglica, ToRglica};

use crate::{
    basic::Button,
    impl_view, view,
    view::{ViewData, ViewFrame, ViewSubviews},
    View, ViewBase, ViewCallbacks,
};

#[view]
#[derive(Default, Debug)]
pub struct DPadView {
    up:           Rglica<Button>,
    down:         Rglica<Button>,
    left:         Rglica<Button>,
    right:        Rglica<Button>,
    pub on_press: Event<Direction>,
}

impl_view!(DPadView);

impl DPadView {
    pub fn set_images(
        &mut self,
        up: Handle<Image>,
        down: Handle<Image>,
        left: Handle<Image>,
        right: Handle<Image>,
    ) -> &mut Self {
        error!("set_images");
        error!("{}", format!("{:?}", self.up));
        self.up.set_image(up);
        error!("UP: OK");
        self.down.set_image(down);
        self.left.set_image(left);
        self.right.set_image(right);
        self
    }
}

impl ViewCallbacks for DPadView {
    fn setup(&mut self) {
        self.up = self.add_view();
        self.down = self.add_view();
        self.left = self.add_view();
        self.right = self.add_view();

        self.up
            .on_tap
            .set(self, |this, _| this.on_press.trigger(Direction::Up));

        self.down
            .on_tap
            .set(self, |this, _| this.on_press.trigger(Direction::Down));

        self.left
            .on_tap
            .set(self, |this, _| this.on_press.trigger(Direction::Left));

        self.right
            .on_tap
            .set(self, |this, _| this.on_press.trigger(Direction::Right));
    }

    fn layout(&mut self) {
        let frame = self.frame();
        let third = frame.width() / 3.0;
        let half = frame.height() / 2.0;

        self.up.set_frame((third, 0, third, half));
        self.down.set_frame((third, half, third, half));
        self.left.set_frame((0, half, third, half));
        self.right.set_frame((third * 2.0, half, third, half));
    }
}
