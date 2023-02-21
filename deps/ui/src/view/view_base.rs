use std::{
    any::Any,
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use gl_image::Image;
use gm::{flat::Rect, Color};
use refs::{Own, ToWeak, Weak};
use rtools::{data_manager::Handle, Unwrap};
use smart_default::SmartDefault;
use vents::Event;

use crate::{layout::Placer, view::view_callbacks::ViewInternalSetup, PathData, Touch, View};

#[derive(SmartDefault)]
pub struct ViewBase {
    pub(crate) color: Color,

    pub(crate) corner_radius: f32,
    pub(crate) border_color:  Color,

    pub(crate) touch_enabled: RefCell<bool>,

    pub(crate) is_hidden: bool,

    pub(crate) frame:          Rect,
    pub(crate) absolute_frame: Rect,

    pub(crate) superview: Weak<dyn View>,
    pub(crate) subviews:  Vec<Own<dyn View>>,

    pub(crate) touch_id: u64,

    pub(crate) image: Handle<Image>,

    pub(crate) is_selected: bool,
    pub(crate) is_deleted:  bool,

    pub label: String,

    pub place:          Unwrap<Placer>,
    pub paths:          Vec<PathData>,
    pub on_touch:       Event<Touch>,
    pub on_touch_began: Event<Touch>,
}

#[derive(Default)]
pub struct BaseView {
    view: ViewBase,
}

impl View for BaseView {
    fn init_views(&mut self) {}

    fn weak_view(&self) -> Weak<dyn View> {
        (self as &dyn View).weak()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ViewInternalSetup for BaseView {
    fn internal_setup(&mut self) {}
}

impl Deref for BaseView {
    type Target = ViewBase;

    fn deref(&self) -> &ViewBase {
        &self.view
    }
}

impl DerefMut for BaseView {
    fn deref_mut(&mut self) -> &mut ViewBase {
        &mut self.view
    }
}
