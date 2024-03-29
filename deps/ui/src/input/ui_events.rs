use gm::flat::Size;
use refs::MainLock;
use vents::Event;

use crate::{Touch, UIEvent};

static UI_EVENTS: MainLock<UIEvents> = MainLock::const_new();

#[derive(Default)]
pub struct UIEvents {
    on_touch:       Event<Touch>,
    on_debug_touch: Event<Touch>,
    size_changed:   Event<Size<u32>>,
    keyboard_input: UIEvent<char>,
}

impl UIEvents {
    pub fn on_touch() -> &'static Event<Touch> {
        &UI_EVENTS.on_touch
    }

    /// Is never disabled
    pub fn on_debug_touch() -> &'static Event<Touch> {
        &UI_EVENTS.on_debug_touch
    }

    pub fn size_changed() -> &'static Event<Size<u32>> {
        &UI_EVENTS.size_changed
    }

    pub fn keyboard_input() -> &'static UIEvent<char> {
        &UI_EVENTS.keyboard_input
    }
}
