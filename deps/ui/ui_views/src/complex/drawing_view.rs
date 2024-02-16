use gm::{
    axis::Axis,
    flat::{Point, Points, Size},
};
use refs::Weak;
use wgpu_wrapper::PathData;
mod test_engine {
    pub(crate) use refs;
    pub(crate) use ui;
}

use ui::{view, ViewFrame, ViewSetup};
use wgpu_wrapper::wgpu::PolygonMode;

#[view]
pub struct DrawingView {
    pub mode:    PolygonMode,
    pub rescale: bool,
    paths:       Vec<PathData>,
}

impl ViewSetup for DrawingView {
    fn setup(mut self: Weak<Self>) {
        self.mode = PolygonMode::Line;
    }
}

impl DrawingView {
    pub fn paths(&self) -> &[PathData] {
        &self.paths
    }

    pub fn add_path<Container, P>(&mut self, path: Container) -> &mut Self
    where
        P: Into<Point>,
        Container: IntoIterator<Item = P>, {
        let points = path.into_iter().map(Into::into).collect();

        let path = self.process_points(points);
        if path.is_empty() {
            return self;
        }
        self.paths.push(path.into());
        self
    }

    fn process_points(&self, path: Points) -> Points {
        if !self.rescale {
            return path;
        }

        let max_x = path.iter().map(|p| p.x).fold(f32::NAN, f32::max);
        let max_y = path.iter().map(|p| p.y).fold(f32::NAN, f32::max);

        let path_size = Size::new(max_x, max_y);

        let fitted_size = path_size.fit_in_rect::<{ Axis::X }>(self.frame()).size;

        let ratios = path_size.ratios(fitted_size);

        path.into_iter().map(|point| point * ratios).collect()
    }

    pub fn remove_all_paths(&mut self) {
        self.paths.clear()
    }
}
