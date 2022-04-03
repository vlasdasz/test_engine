use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use gl_image::Image;
use gm::Point;
use rtools::Rglica;

use crate::{Level, Sprite, SpriteBase};

#[derive(Debug)]
pub struct Weapon {
    sprite:              SpriteBase,
    pub(crate) velocity: Point,
    pub bullet_speed:    f32,
    pub bullet_image:    Option<Image>,
}

impl Weapon {
    pub fn new(level: Rglica<dyn Level>) -> Self {
        let mut sprite: SpriteBase = (0, 0, 2365.0 / 1000.0, 854.0 / 1000.0).into();
        sprite.level = level;
        Self {
            sprite,
            velocity: Default::default(),
            bullet_speed: 1.0,
            bullet_image: None,
        }
    }

    pub fn shoot_at(&mut self, pos: Point) {
        let vector = (pos - self.position()).normalized();
        let pos = self.position() + vector * 3.2;

        let vel = vector * self.bullet_speed + self.velocity;

        let mut body = self.level_mut().add_body((pos.x, pos.y, 0.8, 0.15).into());
        body.set_rotation(self.rotation());
        body.set_velocity(vel);
        body.sprite_mut().tag = "bullet".into();

        if let Some(image) = &self.bullet_image {
            body.set_image(image.clone())
        }
    }
}

impl Sprite for Weapon {
    fn sprite(&self) -> &SpriteBase {
        &self.sprite
    }

    fn sprite_mut(&mut self) -> &mut SpriteBase {
        &mut self.sprite
    }
}

impl Deref for Weapon {
    type Target = SpriteBase;
    fn deref(&self) -> &SpriteBase {
        &self.sprite
    }
}

impl DerefMut for Weapon {
    fn deref_mut(&mut self) -> &mut SpriteBase {
        &mut self.sprite
    }
}
