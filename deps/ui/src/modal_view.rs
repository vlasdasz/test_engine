use gm::flat::Size;
use refs::{Own, ToWeak, Weak};
use vents::Event;

use crate::{view::ViewSubviews, UIManager, View};

pub trait ModalView<In = (), Out: 'static = ()>: 'static + View + Default {
    fn show_modally(input: In, callback: impl FnOnce(Out) + 'static) {
        let view = Own::<Self>::default();
        let size = Self::modal_size();
        let weak = view.weak();
        UIManager::root_view().add_subview(view);
        weak.setup_input(input);
        weak.place.center().size(size.width, size.height);
        UIManager::push_touch_view(weak.weak_view());
        weak.modal_event().once(callback);
    }

    fn hide_modal(mut self: Weak<Self>, result: Out) {
        self.remove_from_superview();
        self.modal_event().trigger(result);
    }

    fn modal_event(&self) -> &Event<Out>;

    fn modal_size() -> Size;

    fn setup_input(self: Weak<Self>, _: In) {}
}
