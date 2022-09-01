use std::ops::Deref;

use gm::flat::Rect;
use rtools::{math::IntoF32, Rglica, ToRglica};

use crate::{
    view::{ViewFrame, ViewSubviews},
    View,
};

#[derive(Default)]
pub struct DeprecatedPlacer {
    view:    Rglica<dyn View>,
    frame:   Rglica<Rect>,
    s_frame: Rglica<Rect>,
}

impl DeprecatedPlacer {
    pub fn make(view: Rglica<dyn View>) -> Self {
        Self {
            view,
            frame: view.frame().to_rglica(),
            s_frame: view.super_frame().to_rglica(),
        }
    }
}

impl DeprecatedPlacer {
    // pub fn background_margin(&mut self, margin: impl IntoF32) {
    //     let margin = margin.into_f32();
    //     self.frame.origin = (margin, margin).into();
    //     self.frame.size = (self.s_width() - margin * 2.0, self.s_height() -
    // margin * 2.0).into(); }

    pub fn center_hor(&mut self) {
        self.frame.origin.x = self.s_width() / 2.0 - self.width() / 2.0;
    }

    pub fn center_ver(&mut self) {
        self.frame.origin.y = self.s_height() / 2.0 - self.height() / 2.0;
    }

    pub fn center(&mut self) {
        self.center_hor();
        self.center_ver();
    }

    pub fn bottom_center(&mut self, margin: impl IntoF32) {
        self.center_hor();
        self.frame.origin.y = self.s_height() - self.height() - margin.into_f32();
    }

    pub fn left_half(&mut self) {
        *self.frame = (0, 0, self.s_width() / 2.0, self.s_height()).into();
    }

    pub fn right_half(&mut self) {
        let half_w = self.s_width() / 2.0;
        *self.frame = (half_w, 0, half_w, self.s_height()).into();
    }

    pub fn right(&mut self) {
        self.center_ver();
        self.frame.origin.x = self.s_width() - self.width();
    }

    pub fn at_center(&mut self, view: &dyn View) {
        self.frame.set_center(view.frame().center())
    }

    pub fn at_bottom(&mut self, view: &dyn View, margin: impl IntoF32) {
        self.at_center(view);
        self.frame.origin.y = view.frame().max_y() + margin.into_f32();
    }

    pub fn at_right(&mut self, view: &dyn View, margin: impl IntoF32) {
        self.at_center(view);
        self.frame.origin.x = view.frame().max_x() + margin.into_f32();
    }

    pub fn frames_for_ratio<const N: usize>(&mut self, ratio: [impl IntoF32; N]) -> [Rect; N] {
        let mut result: [Rect; N] = [Default::default(); N];

        let total_ratio: f32 = ratio.iter().map(|a| a.into_f32()).sum();
        let total_ratio = 1.0 / total_ratio;
        let mut prev_y = 0.0;

        for (i, frame) in result.iter_mut().enumerate() {
            *frame = (
                0,
                if i == 0 { 0.0 } else { prev_y },
                self.width(),
                ratio[i].into_f32() * self.height() * total_ratio,
            )
                .into();
            prev_y = frame.max_y()
        }

        result
    }

    pub fn all_vertically_with_ratio<const N: usize>(&mut self, ratio: [impl IntoF32; N]) {
        debug_assert!(self.subviews().len() == ratio.len());
        let frames = self.frames_for_ratio(ratio);
        for (view, rect) in self.subviews_mut().iter_mut().zip(frames) {
            view.set_frame(rect);
        }
    }

    pub fn anchor(
        &mut self,
        view: impl Deref<Target = impl View + ?Sized>,
        anchor: Anchor,
        position: Anchor,
        margin: impl IntoF32,
    ) {
        let margin = margin.into_f32();

        match anchor {
            Anchor::Top => {
                self.frame.origin.y = view.y() - margin - self.height();
            }
            Anchor::Bot => {
                self.frame.origin.y = view.max_y() + margin;
            }
            Anchor::Left => {
                self.frame.origin.x = view.x() - margin - self.width();
            }
            Anchor::Right => {
                self.frame.origin.x = view.max_x() + margin;
            }
            Anchor::Center => {
                self.frame.origin.x = view.x() - view.height() / 2.0 + self.height() / 2.0;
                self.frame.origin.y = view.y() - view.width() / 2.0 + self.width() / 2.0;
            }
            _ => (),
        }

        match position {
            Anchor::Top => {
                self.frame.origin.y = view.y();
            }
            Anchor::Bot => {
                self.frame.origin.y = view.max_y() - self.height();
            }
            Anchor::Left => {
                self.frame.origin.x = view.x();
            }
            Anchor::Right => {
                self.frame.origin.x = view.max_x() - self.width();
            }
            Anchor::Center => {
                if anchor.is_horizontal() {
                    self.frame.origin.y = view.y() + view.height() / 2.0 - self.height() / 2.0;
                } else {
                    self.frame.origin.x = view.x() + view.width() / 2.0 - self.width() / 2.0;
                }
            }
            _ => (),
        }
    }
}

impl DeprecatedPlacer {
    pub fn proportional_width(&mut self, ratio: impl IntoF32) -> &mut Self {
        self.frame.size.width = self.s_width() * ratio.into_f32();
        self
    }

    pub fn proportional_height(&mut self, ratio: impl IntoF32) -> &mut Self {
        self.frame.size.height = self.s_height() * ratio.into_f32();
        self
    }
}

impl DeprecatedPlacer {
    fn width(&self) -> f32 {
        self.frame.width()
    }

    fn height(&self) -> f32 {
        self.frame.height()
    }

    fn s_width(&self) -> f32 {
        self.s_frame.width()
    }

    fn s_height(&self) -> f32 {
        self.s_frame.height()
    }

    fn subviews(&self) -> &[Box<dyn View>] {
        self.view.subviews()
    }

    fn subviews_mut(&mut self) -> &mut [Box<dyn View>] {
        self.view.subviews_mut()
    }
}
