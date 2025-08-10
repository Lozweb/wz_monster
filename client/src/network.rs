use bevy::prelude::{Entity, Resource, SystemSet};
use bevy_renet2::prelude::ClientId;
use std::collections::HashMap;

pub mod system;

#[derive(Default, Resource)]
pub struct NetworkMapping(pub(crate) HashMap<Entity, Entity>);

#[derive(Debug)]
struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}

#[derive(Debug, Default, Resource)]
pub struct ClientLobby {
    players: HashMap<ClientId, PlayerInfo>,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;

