use std::net::TcpStream;
use std::io::Read;

use bevy::{
    prelude::*, 
    utils::hashbrown::HashMap
};
use iyes_loopless::prelude::*;
use serde::{Serialize, Deserialize};

use kesko_core::event::{
    SystemRequestEvent,
};
use kesko_physics::event::PhysicRequestEvent;
use kesko_models::{
    Model, SpawnEvent
};

use super::{
    TcpBuffer,
    TcpConnectionState
};



#[derive(Deserialize, Serialize, Debug)]
pub(crate) enum TcpCommand {
    Close,
    GetState,
    SpawnModel {
        model: Model,
        position: Vec3,
        color: Color
    },
    Despawn {
        id: u64
    },
    DespawnAll,

    ApplyMotorCommand {
        id: u64,
        command: HashMap<String, f32>
    },
    PausePhysics,
    RunPhysics,
    IsAlive
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct HttpRequest {
    pub(crate) commands: Vec<TcpCommand>
}

impl HttpRequest {
    pub(crate) fn parse(request_str: String) -> Result<String, String> {
        for line in request_str.lines() {
            if line.starts_with("{") {
                return Ok(line.to_owned());
            }
        }
        Err("Failed to parse http request".to_owned())
    }

    pub(crate) fn from_http_str(req: String) -> Result<HttpRequest, String> {
        match Self::parse(req) {
            Ok(json) =>{
                match serde_json::from_str::<HttpRequest>(json.as_str()) {
                    Ok(req) => Ok(req),
                    Err(e) => Err(format!("Failed to convert http request to SimHttpRequest: {}", e))
                }
            }
            Err(e) => Err(format!("{}", e))
        }
    }
}

pub(crate) fn handle_requests(
    mut commands: Commands,
    mut tcp_stream: ResMut<TcpStream>,
    mut tcp_buffer: ResMut<TcpBuffer>,
    mut system_event_writer: EventWriter<SystemRequestEvent>,
    mut spawn_event_writer: EventWriter<SpawnEvent>,
    mut physic_event_writer: EventWriter<PhysicRequestEvent>
) {

    let mut got_msg = false;
    while !got_msg {
        match tcp_stream.read(&mut tcp_buffer.data) {
            Ok(msg_len) => {

                if msg_len == 0 {
                    continue;
                }

                got_msg = true;

                let http_str = String::from_utf8_lossy(&tcp_buffer.data[..msg_len]).to_string();
                match HttpRequest::from_http_str(http_str) {
                    Ok(mut request) => {
                        debug!("Got Request: {:?}", request.commands);

                        for command in request.commands.drain(..) {
                            match command {
                                TcpCommand::Close => system_event_writer.send(SystemRequestEvent::ExitApp),
                                TcpCommand::SpawnModel { model, position, color } => {
                                    spawn_event_writer.send(SpawnEvent::Spawn { model, transform: Transform::from_translation(position), color });
                                },
                                TcpCommand::GetState => system_event_writer.send(SystemRequestEvent::GetState),
                                TcpCommand::PausePhysics => physic_event_writer.send(PhysicRequestEvent::PausePhysics),
                                TcpCommand::RunPhysics => physic_event_writer.send(PhysicRequestEvent::RunPhysics),
                                TcpCommand::IsAlive => system_event_writer.send(SystemRequestEvent::IsAlive),
                                TcpCommand::ApplyMotorCommand { id, command } => system_event_writer.send( SystemRequestEvent::ApplyMotorCommand { id, command }),
                                TcpCommand::Despawn { id } => physic_event_writer.send(PhysicRequestEvent::DespawnBody(id)),
                                TcpCommand::DespawnAll => physic_event_writer.send(PhysicRequestEvent::DespawnAll)
                            }
                        }
                    }
                    Err(e) => error!("{}", e)
                }
            },
            Err(e) => {
                error!("Could not read tcp stream: {}", e);
                commands.insert_resource(NextState(TcpConnectionState::NotConnected));
            }
        }
    }
}
