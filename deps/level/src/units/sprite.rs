use std::{
    any::type_name,
    ops::{Deref, DerefMut},
};

use gm::{
    flat::{Point, Shape, Size},
    Color, ToF32,
};
use rapier2d::{
    dynamics::RigidBodyHandle,
    geometry::{Collider, ColliderHandle},
    pipeline::ActiveEvents,
    prelude::{CoefficientCombineRule, RigidBody, Rotation},
};
use refs::{weak_from_ref, Address, Own};
use wgpu_wrapper::image::ToImage;

use crate::{LevelManager, SpriteData};

pub trait Sprite: Deref<Target = SpriteData> + DerefMut {
    fn make(shape: Shape, position: Point) -> Own<Self>
    where Self: Sized;

    fn update(&mut self) {}

    fn size(&self) -> Size {
        self.size
    }

    fn rigid_handle(&self) -> Option<RigidBodyHandle> {
        None
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        None
    }

    fn position(&self) -> Point {
        if let Some(handle) = self.rigid_handle() {
            let rigid_body = LevelManager::get_rigid_body(handle);
            let translation = rigid_body.translation();
            return Point::new(translation.x, translation.y);
        }
        self.position
    }

    fn set_x(&mut self, x: f32) {
        let mut pos = self.position();
        pos.x = x;
        self.set_position(pos);
    }

    fn set_y(&mut self, y: f32) {
        let mut pos = self.position();
        pos.y = y;
        self.set_position(pos);
    }

    fn rotation(&self) -> f32 {
        if let Some(handle) = self.rigid_handle() {
            LevelManager::get_rigid_body(handle).rotation().angle()
        } else if let Some(handle) = self.collider_handle() {
            LevelManager::get_collider(handle).rotation().angle()
        } else {
            self.rotation
        }
    }

    fn restitution(&mut self) -> f32 {
        self.collider().restitution()
    }

    fn rigid_body(&self) -> &RigidBody {
        unsafe {
            &LevelManager::level_unchecked().sets.rigid_bodies
                [self.rigid_handle().expect("This sprite doesn't have rigid body")]
        }
    }

    fn rigid_body_mut(&mut self) -> &mut RigidBody {
        let handle = self.rigid_handle().expect("This sprite doesn't have rigid body");
        unsafe { &mut LevelManager::level_unchecked().sets.rigid_bodies[handle] }
    }

    fn collider(&self) -> &Collider {
        unsafe {
            &LevelManager::level_unchecked().sets.colliders
                [self.collider_handle().expect("This sprite doesn't have collider")]
        }
    }

    fn collider_mut(&mut self) -> &mut Collider {
        let handle = self.collider_handle().expect("This sprite doesn't have collider");
        unsafe { &mut LevelManager::level_unchecked().sets.colliders[handle] }
    }

    fn enable_collision_detection(&mut self)
    where Self: Sized + 'static {
        assert!(
            self.collider_handle().is_some(),
            "{} doesn't have a collider.",
            type_name::<Self>()
        );
        self.collider_mut().set_active_events(ActiveEvents::COLLISION_EVENTS);
        let weak = weak_from_ref(self);
        LevelManager::level_weak().colliding_sprites.push(weak);
    }

    fn contains(&self, point: Point) -> bool {
        let pos = self.position();
        let size = self.size();
        point.x >= pos.x - size.width
            && point.x <= pos.x + size.width
            && point.y >= pos.y - size.height
            && point.y <= pos.y + size.height
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn is_selected(&self) -> bool {
        self.is_selected
    }

    fn remove(&mut self) {
        let address = self.address();
        LevelManager::level_weak().remove(address);
    }
}

pub trait SpriteTemplates {
    fn set_color(&mut self, _: Color) -> &mut Self;
    fn set_selected(&mut self, _: bool) -> &mut Self;
    fn set_image(&mut self, _: impl ToImage) -> &mut Self;
    fn set_friction(&mut self, friction: impl ToF32) -> &mut Self;
    fn set_restitution(&mut self, _: f32, _: CoefficientCombineRule) -> &mut Self;
    fn set_position(&mut self, _: impl Into<Point>) -> &mut Self;
    fn set_rotation(&mut self, _: impl ToF32) -> &mut Self;
}

impl<T: ?Sized + Sprite> SpriteTemplates for T {
    fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.is_selected = selected;
        self
    }

    fn set_image(&mut self, image: impl ToImage) -> &mut Self {
        self.image = image.to_image();
        self
    }

    fn set_friction(&mut self, friction: impl ToF32) -> &mut Self {
        self.collider_mut().set_friction(friction.to_f32());
        self
    }

    fn set_restitution(&mut self, res: f32, rule: CoefficientCombineRule) -> &mut Self {
        self.collider_mut().set_restitution(res);
        self.collider_mut().set_restitution_combine_rule(rule);
        self
    }

    fn set_position(&mut self, pos: impl Into<Point>) -> &mut Self {
        let pos = pos.into();
        if self.collider_handle().is_some() {
            self.collider_mut().set_position([pos.x, pos.y].into());
        } else if self.rigid_handle().is_some() {
            self.rigid_body_mut().set_position([pos.x, pos.y].into(), true);
        }
        self.position = pos;
        self
    }

    fn set_rotation(&mut self, rotation: impl ToF32) -> &mut Self {
        let rotation = rotation.to_f32();
        if self.rigid_handle().is_some() {
            self.rigid_body_mut().set_rotation(Rotation::new(rotation), true);
        }
        if self.collider_handle().is_some() {
            self.collider_mut().set_rotation(Rotation::new(rotation));
        } else {
            self.rotation = rotation;
        }
        self
    }
}
