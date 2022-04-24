use gm::flat::{Point, Rect};
use rtools::IntoF32;

use crate::{
    placer::Placer,
    view::{ViewInternal, ViewSubviews},
    View,
};

pub trait ViewFrame {
    fn frame(&self) -> &Rect;
    fn super_frame(&self) -> &Rect;
    fn absolute_frame(&self) -> &Rect;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn max_x(&self) -> f32;
    fn max_y(&self) -> f32;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn set_y(&mut self, y: impl IntoF32) -> &mut Self;
    fn set_origin(&mut self, origin: impl Into<Point>) -> &mut Self;
    fn set_center(&mut self, center: impl Into<Point>) -> &mut Self;
    fn set_frame(&mut self, rect: impl Into<Rect>) -> &mut Self;
    fn place(&mut self) -> &mut Placer;
    fn calculate_frames(&mut self);
}

impl<T: ?Sized + View> ViewFrame for T {
    fn frame(&self) -> &Rect {
        &self.view().frame
    }

    fn super_frame(&self) -> &Rect {
        if self.view().superview.is_ok() {
            return self.view().superview.frame();
        }
        self.frame()
    }

    fn absolute_frame(&self) -> &Rect {
        &self.view().absolute_frame
    }

    fn x(&self) -> f32 {
        self.frame().origin.x
    }

    fn y(&self) -> f32 {
        self.frame().origin.y
    }

    fn max_x(&self) -> f32 {
        self.frame().max_x()
    }

    fn max_y(&self) -> f32 {
        self.frame().max_y()
    }

    fn width(&self) -> f32 {
        self.frame().size.width
    }

    fn height(&self) -> f32 {
        self.frame().size.height
    }

    fn set_y(&mut self, y: impl IntoF32) -> &mut Self {
        self.view_mut().frame.origin.y = y.into_f32();
        self
    }

    fn set_origin(&mut self, origin: impl Into<Point>) -> &mut Self {
        self.view_mut().frame.origin = origin.into();
        self
    }

    fn set_center(&mut self, center: impl Into<Point>) -> &mut Self {
        self.view_mut().frame.set_center(center.into());
        self
    }

    fn set_frame(&mut self, rect: impl Into<Rect>) -> &mut Self {
        self.view_mut().frame = rect.into();
        self
    }

    fn place(&mut self) -> &mut Placer {
        &mut self.view_mut().placer
    }

    fn calculate_frames(&mut self) {
        let view = self.view_mut();
        view.absolute_frame = view.frame;
        view.absolute_frame.origin += view.super_absolute_frame().origin;
        self.layout();
        for view in self.subviews_mut() {
            view.calculate_frames();
        }
    }
}
