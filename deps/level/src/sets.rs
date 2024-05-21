use rapier2d::prelude::{ColliderSet, RigidBodySet};

#[derive(Default)]
pub(crate) struct Sets {
    pub(crate) rigid_body: RigidBodySet,
    pub(crate) collider:   ColliderSet,
}