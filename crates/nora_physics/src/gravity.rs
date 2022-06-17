use bevy::prelude::*;
use rapier3d::dynamics::RigidBodySet;
use crate::rigid_body::RigidBodyHandle;


pub struct Gravity(Vec3);

impl Gravity {
    pub fn new(vec: Vec3) -> Self {
        Self(vec)
    }
    pub fn get(&self) -> &Vec3 {
        &self.0
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

#[derive(Component)]
pub struct GravityScale {
    pub val: f32,
    pub initial_val: f32
}

impl Default for GravityScale {
    fn default() -> Self {
        GravityScale { val: 1.0 , initial_val: 1.0}
    }
}

impl GravityScale {
    pub fn new(val: f32) -> Self {
        Self { val, initial_val: val }
    }

    pub fn reset(&mut self) {
        self.val = self.initial_val;
    }
}

pub(crate) fn update_gravity_scale_system(
    mut rigid_bodies: ResMut<RigidBodySet>,
    query: Query<(&RigidBodyHandle, &GravityScale), Changed<GravityScale>>
) {
    for (body_handle, gravity_scale) in query.iter() {
        if let Some(body) = rigid_bodies.get_mut(body_handle.0) {
            body.set_gravity_scale(gravity_scale.val, true);
        }
    }
}