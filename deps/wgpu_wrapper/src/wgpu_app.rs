use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use anyhow::Result;
use dispatch::on_main;
use gm::flat::Size;
use log::{error, trace};
use refs::{MainLock, Rglica};
use tokio::sync::oneshot::Receiver;
use wgpu::{BindGroupLayout, Device, Queue};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use crate::{
    app::App,
    render::state::{State, DEVICE, QUEUE},
    Screenshot,
};

static APP: MainLock<Option<WGPUApp>> = MainLock::new();

pub struct WGPUApp {
    pub state:  State,
    event_loop: Option<EventLoop<()>>,
    close:      AtomicBool,
}

impl WGPUApp {
    pub(crate) fn current() -> &'static mut Self {
        APP.get_mut().as_mut().expect("App has not been initialized yet.")
    }

    pub fn device() -> &'static Device {
        DEVICE.get_mut().as_mut().expect("Device has not been initialized yet.")
    }

    pub fn queue() -> &'static Queue {
        QUEUE.get_mut().as_mut().expect("Queue has not been initialized yet.")
    }

    pub fn close() {
        on_main(|| {
            Self::current().close.store(true, Ordering::Relaxed);
        });
    }

    pub async fn start(app: Box<dyn App>) -> Result<()> {
        let event_loop = EventLoop::new()?;

        let window = Arc::new(WindowBuilder::new().with_title("Test Engine").build(&event_loop)?);

        let state = State::new(app, window.clone()).await?;

        assert!(APP.is_none(), "Another instance of App already exists.");

        *APP.get_mut() = Self {
            state,
            event_loop: event_loop.into(),
            close: Default::default(),
        }
        .into();

        let app = Self::current();

        app.state.app.set_wgpu_app(Rglica::from_ref(app));
        app.state.app.window_ready();
        app.start_event_loop()
    }

    fn start_event_loop(&mut self) -> Result<()> {
        self.event_loop.take().unwrap().run(|event, elwt| match event {
            Event::WindowEvent { event, window_id } if window_id == self.state.window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::CursorMoved { position, .. } => {
                    if self.state.app.mouse_moved((position.x, position.y).into()) {
                        self.state.window.request_redraw();
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if self.state.app.mouse_event(state, button) {
                        self.state.window.request_redraw();
                    }
                }
                WindowEvent::Touch(touch) => {
                    if self.state.app.touch_event(touch) {
                        self.state.window.request_redraw();
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    trace!("{delta:?}");
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                        elwt.exit()
                    }
                    self.state.app.key_event(event);
                }
                WindowEvent::Resized(physical_size) => {
                    self.state.resize(physical_size);
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    inner_size_writer,
                } => {
                    dbg!(&scale_factor);
                    dbg!(&inner_size_writer);
                    //state.resize(**new_inner_size);
                }
                WindowEvent::RedrawRequested => {
                    if self.close.load(Ordering::Relaxed) {
                        elwt.exit();
                    }

                    self.state.update();

                    match self.state.render() {
                        Ok(()) => {}
                        // Err(wgpu::SurfaceError::Lost) => self.state.resize(self.state.size),
                        // Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                        Err(e) => error!("Render error: {e:?}"),
                    };
                    // TODO: think about good redraw strategy
                    self.state.window.request_redraw();
                }
                _ => {}
            },
            _ => {}
        })?;

        Ok(())
    }

    pub fn set_title(&self, title: impl ToString) {
        self.state.window.set_title(&title.to_string());
    }

    pub fn set_window_size(&self, size: impl Into<Size<u32>>) {
        let size = size.into();
        let _ = self.state.window.request_inner_size(PhysicalSize::new(size.width, size.height));
    }

    pub fn request_read_display(&self) -> Receiver<Screenshot> {
        self.state.request_read_display()
    }

    pub fn path_layout() -> &'static BindGroupLayout {
        &Self::current().state.drawer.path_state.color_size_layout
    }

    pub fn fps(&self) -> f32 {
        self.state.fps
    }
}
