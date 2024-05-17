use std::ops::{Deref, DerefMut};

use dispatch::{from_main, wait_for_next_frame};
use gm::{
    flat::{Rect, Size},
    Color,
};
use log::{trace, warn};
use manage::data_manager::DataManager;
use refs::{weak_from_ref, Own, Weak};
use ui::{
    DrawingView, ImageView, Label, UIManager, View, ViewAnimation, ViewData, ViewFrame, ViewLayout,
    ViewSetup, ViewSubviews, ViewTest,
};
use wgpu::RenderPass;
use wgpu_text::glyph_brush::{BuiltInLineBreaker, HorizontalAlign, Layout, Section, Text, VerticalAlign};
use wgpu_wrapper::WGPUDrawer;

use crate::{ui::ui_test::state::clear_state, App};

pub struct UI;

impl UI {
    pub(crate) fn update() {
        Self::update_view(UIManager::root_view_mut().deref_mut())
    }

    fn update_view(view: &mut dyn View) {
        if view.is_hidden() {
            return;
        }
        view.layout();
        view.commit_animations();
        view.calculate_absolute_frame();
        view.update();
        view.trigger_events();
        for mut view in view.subviews_mut() {
            Self::update_view(view.deref_mut());
        }
    }

    pub(crate) fn draw_view<'a>(
        pass: &mut RenderPass<'a>,
        drawer: &'a WGPUDrawer,
        view: &'a dyn View,
        sections: &mut Vec<Section<'a>>,
        text_offset: &mut f32,
    ) {
        const DRAW_DEBUG_FRAMES: bool = false;

        if view.is_hidden() {
            return;
        }

        if view.absolute_frame().size.is_invalid() {
            warn!(
                "View has invalid frame: {}. Frame: {:?} ",
                view.label(),
                view.frame()
            );
            return;
        }

        let frame = Self::rescale_frame(view.absolute_frame(), 1.0);

        let root_size = UI::root_view_size();

        let clamped_frame = frame.clamp_to(root_size);

        if view.color().a > 0.0 {
            drawer.fill_rect(
                pass,
                &clamped_frame,
                view.color(),
                view.z_position() + *text_offset,
            );
        }

        if let Some(image_view) = view.as_any().downcast_ref::<ImageView>() {
            if image_view.image().was_initialized() {
                weak_from_ref(image_view).check_cropped(&clamped_frame);

                let image = image_view.image();
                // let size: Size = image.size.into();
                // let frame = &size.fit_in_rect::<{ Axis::X }>(view.absolute_frame());
                // let frame = Self::rescale_frame(frame, 1.0, drawer.window_size);

                drawer.draw_image(
                    pass,
                    image.get_static(),
                    &clamped_frame,
                    image_view.cropped(),
                    view.z_position() - UIManager::additional_z_offset(),
                );
            } else {
                warn!("Image is not OK");
            }
        } else if let Some(label) = view.as_any().downcast_ref::<Label>()
            && !label.text.is_empty()
        {
            let center = frame.center();

            let section = Section::default()
                .add_text(
                    Text::new(&label.text)
                        .with_scale(label.text_size())
                        .with_color(label.text_color().as_slice())
                        .with_z(view.z_position() - UIManager::additional_z_offset() + *text_offset),
                )
                .with_bounds((frame.width(), frame.height()))
                .with_layout(
                    Layout::default()
                        .v_align(VerticalAlign::Center)
                        .h_align(HorizontalAlign::Center)
                        .line_breaker(BuiltInLineBreaker::UnicodeLineBreaker),
                )
                .with_screen_position((center.x, center.y));

            *text_offset += UIManager::additional_z_offset();

            sections.push(section);
        } else if let Some(drawing_view) = view.as_any().downcast_ref::<DrawingView>() {
            for path in drawing_view.paths().iter().rev() {
                drawer.draw_buffer(
                    pass,
                    &clamped_frame,
                    path.buffer(),
                    path.bind_group(),
                    path.vertex_range(),
                    drawing_view.z_position() - UIManager::additional_z_offset(),
                );
            }
        }

        if DRAW_DEBUG_FRAMES
            && clamped_frame.size.is_valid()
            && clamped_frame.x() + 2.0 <= root_size.width
            && clamped_frame.y() + 2.0 <= root_size.height
        {
            drawer.outline_rect(
                pass,
                &clamped_frame,
                &Color::TURQUOISE,
                view.z_position() - 0.2,
                2.0,
            );
        }

        let mut text_offset = 0.0;

        for view in view.subviews().iter().rev() {
            let root_frame = UIManager::root_view().frame();
            if view.dont_hide() || view.absolute_frame().intersects(root_frame) {
                Self::draw_view(pass, drawer, view.deref(), sections, &mut text_offset)
            }
        }
    }

    pub fn root_view_size() -> Size {
        UIManager::root_view().size()
    }

    fn rescale_frame(rect: &Rect, display_scale: f32) -> Rect {
        rect * display_scale
    }
}

impl UI {
    pub async fn init_test_view<T: View + ViewTest + Default + 'static>() -> Weak<T> {
        Self::set_test_view(T::new(), 600, 600).await
    }

    pub async fn set_test_view<T: View + 'static>(view: Own<T>, width: u32, height: u32) -> Weak<T> {
        clear_state();

        App::set_window_size((width, height));
        wait_for_next_frame().await;
        let view = from_main(move || {
            let weak = view.weak();
            let mut root = UIManager::root_view_mut();
            root.remove_all_subviews();
            let view = root.__add_subview_internal(view, true);
            view.place().back();
            trace!("{width} - {height}");
            weak
        })
        .await;
        wait_for_next_frame().await;
        view
    }
}