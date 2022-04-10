use gm::flat::{Point, Shape};
use rapier2d::{na::Vector2, prelude::RigidBodyBuilder};
use rtools::Rglica;

use crate::{control::Control, Level, Sprite, SpriteData, ToCollider};

#[derive(Debug)]
pub struct Body {
    sprite: SpriteData,
}

impl Body {
    pub fn velocity(&self) -> Point {
        let vel = self.rigid_body().linvel();
        (vel.x, vel.y).into()
    }

    pub fn set_velocity(&mut self, vel: Point) {
        self.rigid_body_mut()
            .set_linvel([vel.x, vel.y].into(), true)
    }

    pub fn set_restitution(&mut self, res: f32) {
        self.collider_mut().set_restitution(res)
    }

    pub fn lock_rotations(&mut self) {
        self.rigid_body_mut().lock_rotations(true, true);
    }

    pub fn unlock_rotation(&mut self) {
        self.rigid_body_mut().lock_rotations(false, true);
    }
}

impl Sprite for Body {
    fn data(&self) -> &SpriteData {
        &self.sprite
    }

    fn data_mut(&mut self) -> &mut SpriteData {
        &mut self.sprite
    }

    fn make(shape: Shape, position: Point, mut level: Rglica<dyn Level>) -> Box<Self>
    where
        Self: Sized,
    {
        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(Vector2::new(position.x, position.y))
            .build();

        let collider = shape.to_collider().build();

        let level_base = level.base_mut();

        let rigid_handle = level_base.sets.rigid_body.insert(rigid_body);

        let collider_handle = level_base.sets.collider.insert_with_parent(
            collider,
            rigid_handle,
            &mut level_base.sets.rigid_body,
        );

        let mut sprite = SpriteData::make(shape, position, level);

        sprite.collider_handle = collider_handle.into();
        sprite.rigid_handle = rigid_handle.into();

        Box::new(Self {
            sprite: sprite.with_level(level),
        })
    }
}

const FORCE: f32 = 10.0;
impl Control for Body {
    fn jump(&mut self) {
        self.add_impulse((0, FORCE).into());
    }

    fn go_left(&mut self) {
        self.add_impulse((-FORCE, 0).into());
    }

    fn go_right(&mut self) {
        self.add_impulse((FORCE, 0).into());
    }

    fn go_down(&mut self) {
        self.add_impulse((0, -FORCE).into());
    }

    fn add_impulse(&mut self, impulse: Point) {
        self.rigid_body_mut()
            .apply_force([impulse.x * 1000.0, impulse.y * 1000.0].into(), true)
    }
}