use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use kesko_core::{
    cursor_tracking::GrabablePlugin,
    interaction::groups::{GroupDynamic, GroupStatic},
    CorePlugin,
};
use kesko_models::ModelPlugin;
use kesko_physics::PhysicsPlugin;
use kesko_plugins::{main_camera::MainCameraPlugin, InteractionPlugin, UIPlugin};
use kesko_tcp::TcpPlugin;

fn main() {
    App::new()
        .add_plugin(CorePlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ModelPlugin)
        .add_plugin(MainCameraPlugin)
        .add_plugin(PhysicsPlugin {
            initial_state: kesko_physics::PhysicState::Stopped,
            ..default()
        })
        .add_plugin(GrabablePlugin::<GroupDynamic>::default())
        .add_plugin(InteractionPlugin::<GroupDynamic>::default())
        .add_plugin(InteractionPlugin::<GroupStatic>::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(TcpPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}
