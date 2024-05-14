use gm::flat::{Point, Rect, Shape};
use refs::Weak;

use crate::{Level, Sprite, SpriteData};

pub trait LevelCreation {
    fn add_sprite<S: 'static + Sprite>(&mut self, _: Shape, _: impl Into<Point>) -> Weak<S>;
    fn add_rect(&mut self, rect: impl Into<Rect>) -> Weak<SpriteData>;
}

impl<T: ?Sized + Level> LevelCreation for T {
    fn add_sprite<S: 'static + Sprite>(&mut self, shape: Shape, position: impl Into<Point>) -> Weak<S> {
        let sprite = S::make(shape, position.into(), self.weak_level());
        let result = sprite.weak();
        self.base_mut().sprites.push(sprite);
        result
    }

    fn add_rect(&mut self, rect: impl Into<Rect>) -> Weak<SpriteData> {
        let rect = rect.into();
        self.add_sprite::<SpriteData>(Shape::Rect(rect.size), rect.origin)
    }
}
