pub mod car;
pub mod snake;
pub mod arena;
pub mod spider;
pub mod sphere;
pub mod wheely;
pub mod humanoid;

use bevy::prelude::*;


pub enum SpawnEvent {
    OpenWindow,
    Spawn {
        model: Model,
        transform: Transform,
        color: Color
    }
}


/// Description on how to manually control a robot
/// The text will be shown in the multibody ui
#[derive(Component)]
pub struct ControlDescription(pub String);


// Enum to represent each default model
#[derive(Debug, PartialEq, Clone)]
pub enum Model {
    Car,
    Snake,
    Spider,
    Sphere,
    Wheely,
    Humanoid
}

impl Model {

    /// Names to use for example in UI
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Car => "Car",
            Self::Snake => "Snake",
            Self::Spider => "Spider",
            Self::Sphere => "Sphere",
            Self::Wheely => "Wheely",
            Self::Humanoid => "Humanoid"
        }
    }
}

/// System to spawn a model given an spawn event
pub fn spawn_model_system(
    mut ui_event_reader: EventReader<SpawnEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in ui_event_reader.iter() {
        if let SpawnEvent::Spawn { model, transform, color} = event {

            let material = materials.add(color.clone().into());

            match model {
                Model::Spider => spider::spawn_spider(&mut commands, material, *transform, &mut meshes),
                Model::Snake => snake::spawn_snake(&mut commands, material, *transform, &mut meshes),
                Model::Car => {
                    let wheel_material = materials.add(Color::DARK_GRAY.into());
                    car::Car::spawn_car(&mut commands, material, wheel_material, *transform, &mut meshes);
                },
                Model::Sphere => sphere::spawn_sphere(&mut commands, material, *transform, &mut meshes),
                Model::Wheely => {
                    let wheel_material = materials.add(Color::DARK_GRAY.into());
                    wheely::Wheely::spawn_wheely(&mut commands, material, wheel_material, *transform, &mut meshes);
                },
                Model::Humanoid => humanoid::Humanoid::spawn(&mut commands, material, *transform, &mut meshes)
            }
        }
    }
}