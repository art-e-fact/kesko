use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use crate::rapier_extern::rapier::prelude as rapier;
use serde::{
    Serialize, Deserialize
};

use crate::{
    rigid_body::{
        RigidBodyHandle, 
        BodyHandle2Entity, 
        RigidBodyName, 
        Entity2BodyHandle
    }, 
    conversions::IntoBevy,
    joint::JointState
};

#[derive(Serialize, Deserialize, Clone)]
pub struct MultiBodyStates(pub Vec<MultiBodyState>);

/// Used for sending data outside of Kesko
#[derive(Serialize, Deserialize, Clone)]
pub struct MultiBodyState {
    pub name: String,
    pub global_position: Vec3,
    pub global_orientation: Vec3,
    pub global_angular_velocity: Vec3,
    pub relative_positions: Option<BTreeMap<String, Vec3>>,
    pub joint_states: Option<BTreeMap<String, Option<JointState>>>
}


/// Component to indicate that the entity is a multibody root entity
#[derive(Component)]
pub struct MultibodyRoot {
    // Name of the multibody
    pub name: String,
    /// Linear velocity of body
    pub linvel: Vec3,
    /// Angular velocity of body
    pub angvel: Vec3,
    /// Map from rigidbody name to entity that has a reference to a joint
    pub child_map: BTreeMap<String, Entity>
}

/// Component to indicate that the entity is a multibody child entity
#[derive(Component)]
pub struct MultibodyChild {
    /// Name of the child link
    pub name: String,
    /// name of the root link
    pub root: Entity,
    /// Map from rigidbody name to entity that has a reference to a joint
    pub child_map: BTreeMap<String, Entity>
}

/// Name registry to ensure all multibodies have a unique name
pub struct MultibodyNameRegistry {
    names: HashMap<String, i32>
}
impl MultibodyNameRegistry {
    pub fn new() -> Self {
        Self {
            names: HashMap::<String, i32>::new()
        }
    }

    pub fn create_unique_name(&mut self, name: &String) -> String {

        if self.names.contains_key(name) {
            *self.names.get_mut(name).expect("Should contain key") += 1;
        } else {
            self.names.insert(name.clone(), 0);
        }
        return format!("{}-{}", name, self.names.get(name).unwrap());
    }
}

/// System that adds components related to multibodies
/// 
/// Todo: Look how to improve this, now it feels a bit complicated and not clear.
#[allow(clippy::type_complexity)]
pub(crate) fn add_multibodies(
    mut commands: Commands,
    mut name_registry: ResMut<MultibodyNameRegistry>,
    multibody_joint_set: Res<rapier::MultibodyJointSet>,
    body_2_entity: Res<BodyHandle2Entity>,
    rigid_body_query: Query<
        (Entity, &RigidBodyHandle, Option<&RigidBodyName>), 
        (With<RigidBodyHandle>, Without<MultibodyChild>, Without<MultibodyRoot>)
    >
) {

    for (entity, handle, body_name) in rigid_body_query.iter() {

        // get the multibodyjoint link for the rigidbody handle.
        if let Some(body_joint_link) = multibody_joint_set.rigid_body_link(handle.0) {

            // get the multibody of the joint link
            if let Some(multibody) = multibody_joint_set.get_multibody(body_joint_link.multibody) {
                
                // build a hash map with names and entity to store in the component.
                // the component can later be used to easy get hold of different links and joints
                let mut joints = BTreeMap::<String, Entity>::new();

                for link in multibody.links() {

                    // don't add root link to root
                    if link.is_root() & multibody.link(body_joint_link.id).expect("link should exists").is_root() {
                        continue;
                    }

                    let link_entity = body_2_entity.get(&link.rigid_body_handle()).expect("body should be in body to entity map");

                    match rigid_body_query.get(*link_entity) {
                        Ok((_, _, name)) => {
                            if let Some(name) = name {
                                joints.insert(name.0.clone(), *link_entity);
                            } else {
                                joints.insert(link_entity.id().to_string(), *link_entity);
                            }
                        },
                        Err(e) => error!("{e}")
                    }
                }

                let mut name = if let Some(body_name) = body_name {
                    body_name.0.clone()
                } else {
                    // if we don't have a name use the entity id
                    entity.id().to_string()
                };

                if handle.0 == multibody.root().rigid_body_handle() {
                    // We have a root
                    // make the name unique
                    name = name_registry.create_unique_name(&name);
                    commands.entity(entity).insert(MultibodyRoot { 
                        name, 
                        linvel: Vec3::ZERO,
                        angvel: Vec3::ZERO,
                        child_map: joints 
                    });
                } else {
                    // not a root
                    let root_rigid_body_handle = multibody.root().rigid_body_handle();
                    let root_entity = body_2_entity.get(&root_rigid_body_handle).expect("Every rigid body should be connected to an entity");
                    commands.entity(entity).insert(MultibodyChild { 
                        name,
                        root: *root_entity, 
                        child_map: joints
                    });
                }
            }
        }
    }
}

pub(crate) fn update_multibody_vel_angvel(
    rigid_body_set: ResMut<rapier::RigidBodySet>,
    e2b: Res<Entity2BodyHandle>,
    mut multibodies: Query<(Entity, &mut MultibodyRoot)>
) {

    for (e, mut multibody) in multibodies.iter_mut() {
        if let Some(handle) = e2b.get(&e) {
            if let Some(rigid_body) = rigid_body_set.get(*handle) {
                multibody.linvel = rigid_body.linvel().into_bevy();
                multibody.angvel = rigid_body.angvel().into_bevy();
            }
        }
    }
}


#[cfg(test)]
mod tests {

}