use anyhow::{anyhow, Result};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_obj::ObjPlugin;
use kesko_core::event::SimulatorRequestEvent;
use kesko_models::SpawnSet;

use crate::{SdfAsset, SdfAssetLoader, convert_sdf_to_components};
extern crate flatbuffers;

// #[allow(dead_code, unused_imports)]
// #[path = "./urdf_generated.rs"]
// mod urdf_generated;
// pub use urdf_generated::kesko::urdf::root_as_spawn_urdf;

// use crate::{convert_urdf_to_components, SdfAsset, SdfAssetLoader, UrdfModel};

pub struct SdfPlugin;

impl Plugin for SdfPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<SdfAsset>()
            .init_asset_loader::<SdfAssetLoader>()
            .add_plugin(ObjPlugin)
            .add_system(convert_sdf_to_components.in_base_set(SpawnSet::Spawn))
            // .add_system(parse_spawn_message.pipe(spawn_urdf))
            ;
    }

    fn name(&self) -> &str {
        "Simulation Description Format loader Plugin"
    }
}

// pub fn parse_spawn_message(
//     mut system_requests: EventReader<SimulatorRequestEvent>,
// ) -> Result<Vec<UrdfModel>> {
//     let messages = system_requests
//         .iter()
//         .map(|event| {
//             if let SimulatorRequestEvent::PublishFlatBuffers(data) = event {
//                 let spawn_urdf = root_as_spawn_urdf(data.as_slice())?;
//                 let urdf_path = spawn_urdf.urdf_path().to_string();
//                 let transform = spawn_urdf
//                     .position()
//                     .and_then(|position| {
//                         Some(Transform::from_xyz(
//                             position.x(),
//                             position.y(),
//                             position.z(),
//                         ))
//                     })
//                     .unwrap_or_default();
//                 let package_mapping = spawn_urdf
//                     .package_mappings()
//                     .unwrap_or_default()
//                     .iter()
//                     .map(|mapping| {
//                         (
//                             mapping.package_name().unwrap_or("").into(),
//                             mapping.package_path().unwrap_or("").into(),
//                         )
//                     })
//                     .collect::<HashMap<String, String>>();

//                 Ok(UrdfModel::new(
//                     urdf_path,
//                     package_mapping,
//                     transform,
//                 ))
//             } else {
//                 Err(anyhow!("Not a SpawnUrdf message"))
//             }
//         })
//         .filter(|message| message.is_ok())
//         .map(|message| message.unwrap())
//         .collect::<Vec<UrdfModel>>();
    
//     Ok(messages)
// }

// pub fn spawn_urdf(
//     In(result): In<Result<Vec<UrdfModel>>>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// )  {
//     if let Ok(models) = result {
//         for model in models {
//             model.spawn(&mut commands, &asset_server);
//         }
//     }
// }