use std::{ops::DerefMut, rc::Rc};

use gl_wrapper::events::Events;
use glfw::{Action, Key};
use gm::Point;
use sprites::{Level, SpritesDrawer};
use tools::{Rglica, ToRglica};
use ui::{
    init_view_on,
    input::touch::{ButtonState, Event},
    Touch, View,
};

use crate::{debug_view::DebugView, ui_drawer::UIDrawer};

pub trait GameView: View {
    fn level(&self) -> &dyn Level;
    fn level_mut(&mut self) -> &mut dyn Level;
    fn set_drawer(&mut self, _drawer: Rc<dyn SpritesDrawer>) {}
}

pub struct UILayer {
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    pub cursor_position: Point,
    pub root_view:       Box<dyn View>,
    pub debug_view:      Rglica<DebugView>,
    pub view:            Rglica<dyn GameView>,

    pub sprites_drawer: Rc<dyn SpritesDrawer>,

    pub drawer: UIDrawer,

    pub events: Rglica<Events>,

    pub fps:        u64,
    pub prev_time:  i64,
    pub frame_time: f64,
}

impl UILayer {
    pub fn on_touch(&mut self, mut touch: Touch) {
        error!("{:?}", touch);
        self.root_view.check_touch(&mut touch);
    }

    pub fn set_view(&mut self, mut view: Box<dyn GameView>) {
        let drawer = self.sprites_drawer.clone();
        view.set_drawer(drawer.clone());
        self.view = view.to_rglica();
        self.root_view.add_subview(view);
        self.view.level_mut().setup();
    }

    pub fn add_debug_view(&mut self) {
        self.debug_view = init_view_on::<DebugView>(self.root_view.deref_mut());
    }
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
impl UILayer {
    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position;
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    Event::Moved,
        });
    }

    fn on_mouse_click(&mut self, _button: glfw::MouseButton, state: Action) {
        self.on_touch(Touch {
            id:       1,
            position: self.cursor_position,
            event:    Event::from_state(ButtonState::from_glfw(state)),
        })
    }

    fn on_key_pressed(&mut self, key: Key, action: Action) {
        if action != Action::Press {
            return;
        }

        self.view
            .level_mut()
            .on_key_pressed(key.get_name().unwrap_or_else({
                || {
                    match key {
                        Key::Space => " ",
                        _ => "unknown",
                    }
                    .into()
                }
            }))
    }

    pub fn setup_events(&mut self) {
        let mut this = self.to_rglica();

        self.events
            .on_key_pressed
            .subscribe(move |a| this.on_key_pressed(a.0, a.1));

        let mut this = self.to_rglica();
        self.events
            .on_mouse_click
            .subscribe(move |a| this.on_mouse_click(a.0, a.1));

        let mut this = self.to_rglica();
        self.events
            .on_cursor_moved
            .subscribe(move |a| this.on_cursor_moved(a));
    }
}
