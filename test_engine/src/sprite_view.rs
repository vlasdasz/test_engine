use rtools::{Rglica, ToRglica};
use sprites::Sprite;
use ui::{
    basic::Button,
    complex::LabeledView,
    view_base::{add_view, ViewBase},
    View,
};

use crate::assets::Assets;

#[derive(Default, Debug)]
pub struct SpriteView {
    base: ViewBase,

    position: Rglica<LabeledView>,
    size:     Rglica<LabeledView>,
    color:    Rglica<LabeledView>,

    delete_button: Rglica<Button>,

    sprite: Rglica<dyn Sprite>,
}

impl SpriteView {
    pub fn set_sprite(&mut self, sprite: Rglica<dyn Sprite>) {
        self.sprite = sprite.clone();
        if sprite.is_null() {
            self.position.clear();
            self.size.clear();
            self.color.clear();
            return;
        }
        self.position.set_value(sprite.position());
        self.size.set_value(sprite.size());
        self.color.set_value(sprite.color());
    }

    fn setup_delete_button(&mut self) {
        self.delete_button = add_view(self);
        self.delete_button.set_image(Assets::image("delete.png"));

        self.delete_button.set_frame((100, 100).into());

        let mut this = self.to_rglica();
        self.delete_button.on_tap.subscribe(move |_| {
            if this.sprite.is_ok() {
                this.sprite.remove();
                this.set_sprite(Rglica::default());
            }
        });
    }
}

impl View for SpriteView {
    fn setup(&mut self) {
        self.position = add_view(self);
        self.size = add_view(self);
        self.color = add_view(self);

        self.position.set_label("position:");
        self.size.set_label("size:");
        self.color.set_label("color:");

        self.setup_delete_button();
    }

    fn layout(&mut self) {
        self.place().all_vertically()
    }

    fn update(&mut self) {
        if self.sprite.is_null() {
            return;
        }
        self.set_sprite(self.sprite.clone());
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
