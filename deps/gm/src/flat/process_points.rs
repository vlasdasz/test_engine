use crate::{
    flat::{Point, Size},
    misc::FloatIterExt,
};

type Points<'a> = Vec<&'a Point>;

pub trait ProcessPoints {
    fn size(&self) -> Size {
        (self.width(), self.height()).into()
    }

    fn width(&self) -> f32 {
        self.max_x() - self.min_x()
    }

    fn height(&self) -> f32 {
        self.max_y() - self.min_y()
    }

    fn max_x(&self) -> f32;
    fn max_y(&self) -> f32;
    fn min_x(&self) -> f32;
    fn min_y(&self) -> f32;
}

impl ProcessPoints for Points<'_> {
    fn max_x(&self) -> f32 {
        self.iter().map(|a| a.x).float_max()
    }

    fn max_y(&self) -> f32 {
        self.iter().map(|a| a.y).float_max()
    }

    fn min_x(&self) -> f32 {
        self.iter().map(|a| a.x).float_min()
    }

    fn min_y(&self) -> f32 {
        self.iter().map(|a| a.y).float_min()
    }
}