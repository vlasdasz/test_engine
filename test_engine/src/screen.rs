use std::{ops::DerefMut, path::PathBuf, ptr::null_mut};

use gl_image::ImageShaders;
use gl_wrapper::{buffers::Buffers, monitor::Monitor, GLWrapper};
#[cfg(desktop)]
use gl_wrapper::{gl_events::GlEvents, GLFWManager};
#[cfg(mobile)]
use gm::volume::GyroData;
use gm::{flat::Size, Color};
use rest::API;
use rtools::Time;
use sprites::{get_sprites_drawer, set_sprites_drawer, Player};
use text::Font;
use ui::{
    layout::Placer,
    refs::{Own, ToWeak, Weak},
    UIManager, View, ViewFrame, ViewInternalSetup,
};

use crate::{
    app_core::TestEngineAction, assets::Assets, sprites_drawer::TESpritesDrawer, ui_drawer::TEUIDrawer,
    ui_layer::UILayer,
};

static mut SCREEN: *mut Screen = null_mut();

pub struct Screen {
    pub ui: Own<UILayer>,

    #[cfg(desktop)]
    glfw:    GLFWManager,
    monitor: Monitor,
}

impl Screen {
    pub fn player(&self) -> Weak<Player> {
        if let Some(level) = &self.ui.level {
            level.player()
        } else {
            Default::default()
        }
    }

    #[cfg(desktop)]
    fn setup_events(&mut self) {
        self.ui.setup_events();

        let mut this = self.weak();
        GlEvents::get().size_changed.val(move |size| {
            this.set_size(size);
        });

        GlEvents::get().frame_drawn.sub(move || {
            this.update();
        });
    }

    fn init(&mut self, #[cfg(desktop)] window_size: Size, view: Own<dyn View>) {
        UIManager::set_display_scale(self.monitor.scale);

        self.ui.debug_view.place = Placer::new(self.ui.debug_view.weak_view()).into();
        self.ui.debug_view.init_views();
        self.ui.debug_view.internal_setup();

        GLWrapper::enable_blend();
        GLWrapper::set_clear_color(Color::GRAY);

        UIManager::set_view(view);

        #[cfg(desktop)]
        {
            self.glfw.set_size(window_size);
            self.set_size(window_size);
        }
    }
}

impl Screen {
    pub fn current() -> &'static mut Screen {
        unsafe {
            assert!(!SCREEN.is_null(), "Assets were not initialized");
            SCREEN.as_mut().unwrap()
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn calculate_fps(&mut self) {
        let now = Time::now();

        let interval = now - self.ui.prev_time;
        self.ui.prev_time = now;

        self.ui.frame_time = interval as f64 / 1_000_000_000.0;
        self.ui.fps = (1.0 / self.ui.frame_time) as u64;

        let fps = self.ui.fps;
        self.ui.debug_view.fps.set(fps);
        if API::is_ok() {
            self.ui.debug_view.weak().set_custom("URL:", API::base_url());
        } else {
            self.ui.debug_view.weak().set_custom("URL:", "API not initizlized");
        }
    }

    pub fn update(&mut self) -> TestEngineAction {
        self.calculate_fps();

        UIManager::drawer().reset_viewport();
        UIManager::set_scheduled();

        GLWrapper::clear();

        if self.ui.level.is_some() {
            self.update_level();
        }

        let mut root_view = UIManager::root_view();

        root_view.set_frame(UIManager::root_view_size());

        UIManager::drawer().update(&mut [root_view, self.ui.debug_view.weak_view()]);

        dispatch::invoke_dispatched();

        #[cfg(desktop)]
        self.glfw.swap_buffers();

        UIManager::update();

        // TODO: tis ugly
        if UIManager::get().close_keyboard {
            UIManager::get().close_keyboard = false;
            TestEngineAction::CloseKeyboard
        } else if UIManager::get().open_keyboard {
            UIManager::get().open_keyboard = false;
            TestEngineAction::OpenKeyboard
        } else {
            TestEngineAction::None
        }
    }

    fn update_level(&mut self) {
        let cursor_position = self.ui.cursor_position;

        let Some(level) = &mut self.ui.level else {
            return;
        };

        level.base_mut().update_physics();
        level.update();

        level.set_cursor_position(cursor_position);

        for sprite in level.sprites_mut() {
            sprite.update();
            sprite.draw();
        }
    }

    pub fn set_size(&mut self, size: impl Into<Size>) {
        let size = size.into();
        trace!("Size changed: {:?}", size);
        UIManager::set_window_size(size);
        get_sprites_drawer().set_resolution(size);
        get_sprites_drawer().set_camera_position((0, 0).into());
        self.update();
    }

    #[cfg(mobile)]
    pub(crate) fn on_gyro_changed(&mut self, gyro: GyroData) {
        error!("GyroData: {:?}", gyro);
        let Some(level) = &mut self.ui.level else {
            return;
        };
        level.on_gyro_changed(gyro);
    }

    #[cfg(desktop)]
    pub fn start_main_loop(&mut self) {
        self.setup_events();
        self.glfw.start_main_loop()
    }
}

impl Screen {
    pub fn new(
        monitor: Monitor,
        assets_path: impl Into<PathBuf>,
        root_view: Own<dyn View>,
        #[cfg(desktop)] glfw: GLFWManager,
        #[cfg(desktop)] window_size: impl Into<Size>,
    ) -> Own<Self> {
        trace!("Creating screen");

        Assets::init(assets_path);
        trace!("Assets: Ok");

        let ui = Own::<UILayer>::default();
        trace!("UILayer: OK");

        UIManager::set_drawer(Own::<TEUIDrawer>::default());
        trace!("UIDrawer: OK");

        set_sprites_drawer(TESpritesDrawer::new());
        trace!("SpritesDrawer: OK");

        let mut screen = Own::new(Self {
            ui,
            #[cfg(desktop)]
            glfw,
            monitor,
        });

        unsafe {
            SCREEN = screen.deref_mut() as *mut Screen;
        }

        Buffers::init(Buffers::default());
        ImageShaders::init(ImageShaders::default());

        screen.init(
            #[cfg(desktop)]
            window_size.into(),
            root_view,
        );

        screen
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        UIManager::drop();
        Font::san_francisco().free();
    }
}
