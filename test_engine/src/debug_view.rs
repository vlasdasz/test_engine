use std::default::default;

use chrono::Utc;
use tools::{platform::Platform, Boxed, Rglica};
use ui::{init_view_on, Label, View, ViewBase};

pub struct DebugView {
    view:              ViewBase,
    fps_label:         Rglica<Label>,
    frame_drawn_label: Rglica<Label>,
    frame_drawn:       u64,
    prev_time:         i64,
    min_fps:           u64,
    max_fps:           u64,
    skipped:           u64,
}

impl View for DebugView {
    fn setup(&mut self) {
        self.frame_mut().size.height = 100.0;
        self.frame_mut().size.width = 280.0;

        self.fps_label = init_view_on(self);
        self.fps_label.set_text("fps label");

        self.frame_drawn_label = init_view_on(self);
        self.frame_drawn_label.set_text("frame drawn label");

        if Platform::MOBILE {
            self.frame_mut().origin.x = 28.0;
            self.frame_mut().origin.y = 28.0;
        }
    }

    fn update(&mut self) {
        self.frame_drawn += 1;
        self.frame_drawn_label
            .set_text(&format!("Frame drawn: {}", self.frame_drawn));

        let now = Utc::now().timestamp_nanos();

        let interval = now - self.prev_time;
        self.prev_time = now;

        let frame_time = interval as f64 / 1000000000.0;
        let fps = (1.0 / frame_time as f64) as u64;

        if self.skipped > 100 {
            if fps < self.min_fps {
                self.min_fps = fps
            }

            if fps > self.max_fps {
                self.max_fps = fps;
            }
        } else {
            self.skipped += 1;
        }

        self.fps_label.set_text(&format!(
            "FPS: {} min: {} max: {}",
            fps, self.min_fps, self.max_fps
        ));
    }

    fn layout(&mut self) {
        self.place().top_left_margin(10);
        self.place().subviews_vertically();
    }

    fn view(&self) -> &ViewBase {
        &self.view
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.view
    }
}

impl Boxed for DebugView {
    fn boxed() -> Box<Self> {
        Box::new(DebugView {
            view:              default(),
            fps_label:         default(),
            frame_drawn_label: default(),
            frame_drawn:       0,
            prev_time:         Utc::now().timestamp_nanos(),
            min_fps:           u64::MAX,
            max_fps:           u64::MIN,
            skipped:           0,
        })
    }
}