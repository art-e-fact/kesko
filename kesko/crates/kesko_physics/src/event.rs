pub mod collision;
pub mod spawn;

use std::collections::BTreeMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use kesko_types::resource::KeskoRes;

use crate::rapier_extern::rapier::prelude as rapier;
use crate::{
    joint::JointInfo,
    multibody::MultibodyRoot,
    rigid_body::{Entity2Body, RigidBodyHandle},
    PhysicState,
};

pub enum PhysicRequestEvent {
    PausePhysics,
    RunPhysics,
    TogglePhysics,
    DespawnBody(u64),
    DespawnAll,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PhysicResponseEvent {
    StartedPhysics,
    StoppedPhysics,
    DespawnedBody(u64),
    DespawnedAllBodies,
    MultibodySpawned {
        id: u64,
        entity: Entity,
        name: String,
        joints: BTreeMap<u64, JointInfo>,
    },
    RigidBodySpawned {
        id: u64,
        name: String,
    },
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn handle_events(
    mut rigid_body_set: ResMut<KeskoRes<rapier::RigidBodySet>>,
    mut collider_set: ResMut<KeskoRes<rapier::ColliderSet>>,
    mut impulse_set: ResMut<KeskoRes<rapier::ImpulseJointSet>>,
    mut multibody_joint_set: ResMut<KeskoRes<rapier::MultibodyJointSet>>,
    mut islands: ResMut<KeskoRes<rapier::IslandManager>>,
    entity_2_body_handle: Res<KeskoRes<Entity2Body>>,
    mut commands: Commands,
    physic_state: Res<State<PhysicState>>,
    mut next_physic_state: ResMut<NextState<PhysicState>>,
    mut request_events: EventReader<PhysicRequestEvent>,
    mut response_events: EventWriter<PhysicResponseEvent>,
    query: Query<(Entity, Option<&MultibodyRoot>), With<RigidBodyHandle>>,
) {
    for event in request_events.iter() {
        match event {
            PhysicRequestEvent::PausePhysics => {
                next_physic_state.set(PhysicState::Stopped);
                response_events.send(PhysicResponseEvent::StoppedPhysics);
            }
            PhysicRequestEvent::RunPhysics => {
                next_physic_state.set(PhysicState::Running);
                response_events.send(PhysicResponseEvent::StartedPhysics);
            }
            PhysicRequestEvent::TogglePhysics => match physic_state.0 {
                PhysicState::Stopped => {
                    next_physic_state.set(PhysicState::Running);
                    response_events.send(PhysicResponseEvent::StartedPhysics);
                }
                PhysicState::Running => {
                    next_physic_state.set(PhysicState::Stopped);
                    response_events.send(PhysicResponseEvent::StoppedPhysics);
                }
            },
            PhysicRequestEvent::DespawnBody(id) => {
                debug!("Despawning body with id {:?}", id);
                let entity = Entity::from_bits(*id);

                match query.get(entity) {
                    Ok((_, root)) => {
                        let mut entities_to_remove = vec![entity];

                        if let Some(root) = root {
                            entities_to_remove.extend(root.child_map.values().cloned());
                        }

                        for entity in entities_to_remove.iter() {
                            debug!("Despawning entity {:?}", entity);
                            commands.entity(*entity).despawn_recursive();

                            if let Some(body_handle) = entity_2_body_handle.get(entity) {
                                despawn_rapier_body(
                                    *body_handle,
                                    &mut rigid_body_set,
                                    &mut islands,
                                    &mut collider_set,
                                    &mut impulse_set,
                                    &mut multibody_joint_set,
                                );
                            }
                        }
                    }
                    Err(e) => error!("Could not get body to remove {}", e),
                }
                response_events.send(PhysicResponseEvent::DespawnedBody(*id))
            }
            PhysicRequestEvent::DespawnAll => {
                query.for_each(|(e, _)| {
                    commands.entity(e).despawn_recursive();
                    if let Some(body_handle) = entity_2_body_handle.get(&e) {
                        despawn_rapier_body(
                            *body_handle,
                            &mut rigid_body_set,
                            &mut islands,
                            &mut collider_set,
                            &mut impulse_set,
                            &mut multibody_joint_set,
                        );
                    }
                });
                response_events.send(PhysicResponseEvent::DespawnedAllBodies);
            }
        }
    }
}

fn despawn_rapier_body(
    handle: rapier::RigidBodyHandle,
    rigid_body_set: &mut rapier::RigidBodySet,
    islands: &mut rapier::IslandManager,
    colliders: &mut rapier::ColliderSet,
    impulse_joints: &mut rapier::ImpulseJointSet,
    multibody_joints: &mut rapier::MultibodyJointSet,
) {
    multibody_joints.remove_multibody_articulations(handle, true);
    rigid_body_set.remove(
        handle,
        islands,
        colliders,
        impulse_joints,
        multibody_joints,
        true,
    );
}
