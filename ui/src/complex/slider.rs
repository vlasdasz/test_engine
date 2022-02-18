use gm::Color;
use rtools::{math::clamped_by, Event, Rglica};

use crate::{
    basic::Circle,
    view_base::{add_view, ViewBase},
    Touch, View,
};

#[derive(Default, Debug)]
pub struct Slider {
    base:          ViewBase,
    circle:        Rglica<Circle>,
    value:         f32,
    multiplier:    f32,
    pub on_change: Event<f32>,
}

impl Slider {
    pub fn set_multiplier(&mut self, multiplier: f32) {
        self.multiplier = multiplier
    }
}

impl View for Slider {
    fn setup(&mut self) {
        self.multiplier = 1.0;
        self.circle = add_view(self);
        let mut circle = self.circle;
        circle.set_frame(self.frame().square().into());
        circle.set_color(Color::BLUE);

        self.enable_touch();
    }

    fn layout(&mut self) {
        self.circle.frame_mut().size = (self.frame().width(), self.frame().width()).into();
    }

    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_ended() {
            return;
        }

        let half_circle = self.circle.frame().height() / 2.0;
        let y_pos = clamped_by(half_circle, self.frame().height() - half_circle, touch.position.y);

        self.circle.frame_mut().origin.y = y_pos - half_circle;

        let value = 1.0 - (y_pos - half_circle) / (self.height() - half_circle * 2.0);
        let value = value * self.multiplier;
        self.value = value;
        self.on_change.trigger(value);
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
