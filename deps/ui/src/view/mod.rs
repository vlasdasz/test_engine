#![allow(clippy::module_inception)]

mod view;
mod view_animation;
mod view_base;
mod view_callbacks;
mod view_controller;
mod view_data;
mod view_frame;
mod view_internal;
mod view_layout;
mod view_subviews;
mod view_touch;
mod view_touch_internal;

pub use view::*;
pub use view_animation::*;
pub use view_base::{BaseView, ViewBase};
pub use view_callbacks::ViewCallbacks;
pub use view_controller::*;
pub use view_data::ViewData;
pub use view_frame::ViewFrame;
pub use view_layout::ViewLayout;
pub use view_subviews::ViewSubviews;
pub use view_touch::ViewTouch;
