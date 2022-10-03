use bevy::app::{App, Plugin};
use kesko_physics::PhysicsPlugin;
use kesko_core::{
    cursor_tracking::GrabablePlugin,
    interaction::groups::GroupDynamic
};


#[derive(Default)]
pub struct DefaultPhysicsPlugin;

impl Plugin for DefaultPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PhysicsPlugin::gravity())
            .add_plugin(GrabablePlugin::<GroupDynamic>::default());
    }
}