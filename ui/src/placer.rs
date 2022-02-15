use std::fmt::{Debug, Formatter};

use gm::Rect;
use rtools::{math::IntoF32, Rglica, ToRglica};

use crate::View;

#[derive(Default)]
pub struct Placer {
    view:    Rglica<dyn View>,
    frame:   Rglica<Rect>,
    s_frame: Rglica<Rect>,
}

impl Placer {
    pub fn make(view: &mut (dyn View + 'static)) -> Self {
        Self {
            view:    view.to_rglica(),
            frame:   view.frame().to_rglica(),
            s_frame: view.super_frame().to_rglica(),
        }
    }
}

impl Placer {
    pub fn as_background(&mut self) {
        *self.frame = self.s_frame.with_zero_origin();
    }

    pub fn background_margin(&mut self, margin: impl IntoF32) {
        let margin = margin.into_f32();
        self.frame.origin = (margin, margin).into();
        self.frame.size = (
            self.s_width() - margin * 2.0,
            self.s_height() - margin * 2.0,
        )
            .into();
    }

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

    pub fn top_left_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = margin.into_f32();
        self.frame.origin.y = margin.into_f32();
    }

    pub fn top_right(&mut self) {
        self.frame.origin.x = self.s_width() - self.width();
        self.frame.origin.y = 0.0;
    }

    pub fn top_right_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = self.s_width() - self.width() - margin.into_f32();
        self.frame.origin.y = margin.into_f32();
    }

    pub fn bottom_left(&mut self) {
        self.frame.origin.x = 0.0;
        self.frame.origin.y = self.s_height() - self.height();
    }

    pub fn bottom_left_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = margin.into_f32();
        self.frame.origin.y = self.s_height() - self.height() - margin.into_f32();
    }

    pub fn bottom_right(&mut self) {
        self.frame.origin.x = self.s_width() - self.width();
        self.frame.origin.y = self.s_height() - self.height();
    }

    pub fn bottom_right_margin(&mut self, margin: impl IntoF32) {
        self.frame.origin.x = self.s_width() - self.width() - margin.into_f32();
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

    pub fn all_vertically(&mut self) {
        let views = self.view.subviews_mut();

        if views.is_empty() {
            return;
        }

        if views.len() == 1 {
            views.last_mut().unwrap().place().as_background();
            return;
        }

        let mut frames: Vec<&mut Rect> = views.iter_mut().map(|a| a.frame_mut()).collect();
        let height: f32 = self.frame.height() / frames.len() as f32;
        let width = self.frame.width();

        for (i, frame) in frames.iter_mut().enumerate() {
            frame.origin.x = 0.0;
            frame.origin.y = i as f32 * height;
            frame.size.width = width;
            frame.size.height = height;
        }
    }

    pub fn all_vertically_with_ratio<T: IntoF32, const N: usize>(&mut self, ratio: &[T; N]) {
        if self.view.subviews().len() != ratio.len() {
            panic!("Invalid ratio len");
        }

        let total_ratio: f32 = ratio.iter().map(|a| a.into_f32()).sum();

        let mut subs: Vec<_> = self
            .view
            .subviews_mut()
            .iter()
            .map(|a| a.to_rglica())
            .collect();

        for (i, view) in subs.iter_mut().enumerate() {
            let is_first = i == 0;
            let prev_index = if is_first { 0 } else { i - 1 };
            let y_pos = if is_first {
                0.0
            } else {
                self.view.subviews()[prev_index].frame().max_y()
            };
            *view.frame_mut() = (
                0,
                y_pos,
                self.view.width(),
                ratio[i].into_f32() * self.view.height() * total_ratio,
            )
                .into();
        }

        // auto tuple = std::forward_as_tuple(views...);
        // auto total_ratio = 1.0f / cu::container::summ(ratio);
        // cu::static_for<size>([&](auto i) {
        //     constexpr bool is_first = i == 0;
        //     auto y_pos = is_first ? 0 : std::get<is_first ? 0 : i -
        // 1>(tuple)->max_y();     std::get<i>(tuple)->edit_frame() =
        //         { 0, y_pos, _frame.size.width, ratio[i] * _frame.size.height
        // * total_ratio, }; });
    }
}

impl Placer {
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
}

impl Debug for Placer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        0.fmt(f)
    }
}
