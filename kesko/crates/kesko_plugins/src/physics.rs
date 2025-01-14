use bevy::app::{App, Plugin};
use kesko_physics::PhysicsPlugin;

#[derive(Default)]
pub struct DefaultPhysicsPlugin;

impl Plugin for DefaultPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin::gravity());
    }
}
