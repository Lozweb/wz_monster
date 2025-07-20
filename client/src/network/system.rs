use crate::network::{ClientLobby, ControlledPlayer, CurrentClientId, NetworkMapping, PlayerInfo};
use bevy::asset::Assets;
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::UVec2;
use bevy::prelude::{Commands, DetectChanges, Entity, Name, Res, ResMut, Sprite, Timer, TimerMode, Transform};
use bevy_renet2::prelude::RenetClient;
use game_core::entities::player::component::{AnimationIndices, AnimationTimer, Grounded, JumpCounter, Player, PlayerInput, PlayerNetwork, PlayerTexture};
use game_core::network::network_entities::{ClientChannel, NetworkedEntities, ServerChannel, ServerMessages};

pub fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
    if player_input.is_changed() {
        let input_message = bincode::serialize(&*player_input).unwrap();
        client.send_message(ClientChannel::Input, input_message);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn client_sync_players(
    player_texture: Res<PlayerTexture>,
    client_id: Res<CurrentClientId>,
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<ClientLobby>,
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

                let texture = player_texture.0.clone();
                let layout = TextureAtlasLayout::from_grid(
                    UVec2::new(32, 48), 4, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);
                let animation_indices = AnimationIndices { first: 1, last: 3 };

                let mut client_entity = commands.spawn((
                    Name::new("Player"),
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
}
pub fn update_player_inputs_from_server(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    network_mapping: ResMut<NetworkMapping>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_texture: Res<PlayerTexture>,
) {
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        for i in 0..networked_entities.entities.len() {
            if let Some(entity) = network_mapping.0.get(&Entity::from_bits(networked_entities.entities[i])) {
                let translation = networked_entities.translations[i].into();
                let transform = Transform {
                    translation,
                    ..Default::default()
                };
                let sprite_index = networked_entities.sprite_index[i];
                let sprite_flip_x = networked_entities.sprite_flip_x[i];

                let layout = TextureAtlasLayout::from_grid(
                    UVec2::new(32, 48), 4, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                commands.entity(*entity)
                    .insert(transform)
                    .insert(Sprite {
                        image: player_texture.0.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_layout,
                            index: sprite_index,
                        }),
                        flip_x: sprite_flip_x,
                        ..Default::default()
                    });
            }
        }
    }
}