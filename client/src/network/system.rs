use crate::network::{ClientLobby, ControlledPlayer, CurrentClientId, NetworkMapping, PlayerInfo};
use bevy::asset::{AssetServer, Assets};
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::UVec2;
use bevy::prelude::{Commands, Entity, Name, Res, ResMut, Sprite, Timer, TimerMode, Transform};
use bevy_renet2::prelude::RenetClient;
use game_core::entities::player::component::{AnimationIndices, AnimationTimer, Grounded, JumpCounter, Player, PlayerInput, PlayerNetwork};
use game_core::network::network_entities::{ClientChannel, NetworkedEntities, ServerChannel, ServerMessages};

pub fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
    let input_message = bincode::serialize(&*player_input).unwrap();
    client.send_message(ClientChannel::Input, input_message);
}

pub fn client_syn_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    client_id: Res<CurrentClientId>,
    mut lobby: ResMut<ClientLobby>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut network_mapping: ResMut<NetworkMapping>,
) {
    let client_id = client_id.0;
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();

        match server_message {
            ServerMessages::PlayerCreate { id, translation, entity } => {
                println!("Player created: {id} at {translation:?}");

                let position = translation.into();

                let texture = asset_server.load("textures/player1.png");
                let layout = TextureAtlasLayout::from_grid(
                    UVec2::new(32, 64), 4, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);
                let animation_indices = AnimationIndices { first: 1, last: 3 };

                let mut client_entity = commands.spawn((
                    Name::new(format!("Player {id}")),
                    Sprite::from_atlas_image(
                        texture,
                        TextureAtlas {
                            layout: texture_atlas_layout,
                            index: animation_indices.first,
                        },
                    ),
                    animation_indices,
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    Player(350.),
                    PlayerInput::default(),
                    Grounded(false),
                    JumpCounter { jumps_left: 2, max_jumps: 2 },
                    Transform::from_translation(position),
                    PlayerNetwork { id },
                ));

                if client_id == id {
                    client_entity.insert(ControlledPlayer);
                }

                let player_info = PlayerInfo {
                    server_entity: Entity::from_bits(entity),
                    client_entity: client_entity.id(),
                };

                lobby.players.insert(id, player_info);
                network_mapping.0.insert(Entity::from_bits(entity), client_entity.id());
            }
            ServerMessages::PlayerRemove { id } => {
                println!("Player removed: {id}");
                if let Some(PlayerInfo {
                                server_entity,
                                client_entity
                            }) = lobby.players.remove(&id) {
                    commands.entity(client_entity).despawn();
                    network_mapping.0.remove(&server_entity);
                }
            }
        }
    }

    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        for i in 0..networked_entities.entities.len() {
            if let Some(entity) = network_mapping.0.get(&Entity::from_bits(networked_entities.entities[i])) {
                let translation = networked_entities.translations[i].into();
                let transform = Transform {
                    translation,
                    ..Default::default()
                };
                commands.entity(*entity).insert(transform);
            }
        }
    }
}