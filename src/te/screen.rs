use crate::gl_wrapper::gl_wrapper::Updatable;
use crate::gl_wrapper::GLWrapper;
use crate::gm::{Color, Point, Rect, Size};
use crate::te::ui::TestView;
use crate::te::{Assets, UIDrawer};
use crate::ui::input::touch::{ButtonState, Event, MouseButton};
use crate::ui::input::Touch;
use crate::ui::view::View;
use crate::ui::ViewBase;
use tools::refs::{make_shared, Shared};
use tools::HasNew;

pub struct Screen<Model: HasNew> {
    cursor_position: Point,
    root_view: Shared<dyn View>,
    ui_drawer: UIDrawer,
    char: u8,
    model: Model,
}

impl<T: HasNew> Screen<T> {
    fn on_touch(&mut self, mut touch: Touch) {
        self.root_view
            .try_borrow_mut()
            .unwrap()
            .check_touch(&mut touch)
    }

    fn update_view(view: Shared<dyn View>) {
        let mut view = view.try_borrow_mut().unwrap();
        view.update();
        for view in view.subviews_mut() {
            Screen::<T>::update_view(view.clone());
        }
    }
}

impl<T: HasNew> Updatable for Screen<T> {
    fn init(&mut self) {
        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(&Color::GRAY);
        let mut debug_view = TestView::new();
        debug_view.font = self.ui_drawer.assets.fonts.default.clone();
        self.root_view
            .try_borrow_mut()
            .unwrap()
            .add_subview(make_shared(debug_view));
        self.root_view
            .try_borrow_mut()
            .unwrap()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
    }

    fn set_size(&mut self, size: Size) {
        self.ui_drawer.set_size(&size);
        self.root_view
            .try_borrow_mut()
            .unwrap()
            .set_frame(Rect::from(size));
        self.update();
    }

    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position
    }

    fn on_mouse_key_pressed(&mut self, _: MouseButton, state: ButtonState) {
        self.on_touch(Touch {
            id: 1,
            position: self.cursor_position,
            event: Event::from_state(state),
        })
    }

    fn update(&mut self) {
        GLWrapper::clear();

        Screen::<T>::update_view(self.root_view.clone());

        self.root_view
            .try_borrow_mut()
            .unwrap()
            .calculate_absolute_frame(&self.ui_drawer.window_size.into());
        self.ui_drawer.draw_view(self.root_view.clone());

        let font = &self.ui_drawer.assets.fonts.default;

        let image = &font.glyph_for_char(self.char as char).image;
        self.char += 1;
        if self.char > 120 {
            self.char = 0;
        }
        let mut rect = Rect::make(10, 10, 20, 20);
        rect.origin = self.ui_drawer.window_size.center();
        let color = Color::WHITE;

        self.ui_drawer.draw_image_in_rect(image, &rect, &color);
    }
}

impl<T: HasNew> HasNew for Screen<T> {
    fn new() -> Screen<T> {
        let assets = Assets::init();
        let ui_drawer = UIDrawer::new(assets);
        Screen {
            cursor_position: Point::new(),
            root_view: make_shared(ViewBase::new()),
            ui_drawer,
            char: 0,
            model: T::new(),
        }
    }
}
