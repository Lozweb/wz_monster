use crate::network::{ClientLobby, ControlledPlayer, CurrentClientId, NetworkMapping, PlayerInfo};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::Vec3;
use bevy::prelude::{ColorMaterial, Commands, Entity, Mesh, Mesh2d, MeshMaterial2d, Name, Rectangle, Res, ResMut, Sprite, Timer, TimerMode, Transform};
use bevy_renet2::prelude::RenetClient;
use game_core::entities::player::component::{AnimationTimer, Grounded, JumpCounter, Player, PlayerInput, PlayerNetwork};
use game_core::entities::player::texture::{texture_entity_to_handle, PlayerTextures, TextureEntity, TextureEntityType};
use game_core::network::network_entities::{NetworkedEntities, ServerChannel, ServerMessages};

#[allow(clippy::too_many_arguments)]
pub fn client_sync_players(
    texture: Res<PlayerTextures>,
    client_id: Res<CurrentClientId>,
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<ClientLobby>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut network_mapping: ResMut<NetworkMapping>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let client_id = client_id.0;
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();

        match server_message {
            ServerMessages::PlayerCreate { id, translation, entity, texture_entity_type } => {
                println!("Player created: {id} at {translation:?}");

                let position = translation.into();
                let rick = TextureEntity::new(&texture_entity_type);
                let (image, layout) =
                    texture_entity_to_handle(&rick.player_texture, &mut texture_atlas_layouts, &texture);

                let client_entity = commands.spawn((
                    Name::new("Player"),
                    Sprite::from_atlas_image(
                        image,
                        TextureAtlas {
                            layout,
                            index: rick.animation_indices.first,
                        },
                    ),
                    rick.animation_indices,
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    Player(350.),
                    PlayerInput::default(),
                    Grounded(false),
                    JumpCounter { jumps_left: 2, max_jumps: 2 },
                    Transform::from_translation(position).with_scale(Vec3::splat(0.5)),
                    PlayerNetwork { id },
                )).id();

                commands.entity(client_entity).with_children(|parent| {
                    parent.spawn((
                        Name::new("Weapons"),
                        Mesh2d(meshes.add(Mesh::from(Rectangle::new(50., 10.)))),
                        MeshMaterial2d(materials.add(Color::BLACK)),
                        // todo on flip x => x = -18.0
                        Transform::from_xyz(18.0, -49.0, 10.0),
                    ));
                });

                if client_id == id {
                    commands.entity(client_entity).insert(ControlledPlayer);
                }

                let player_info = PlayerInfo {
                    server_entity: Entity::from_bits(entity),
                    client_entity,
                };

                lobby.players.insert(id, player_info);
                network_mapping.0.insert(Entity::from_bits(entity), client_entity);
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
    player_textures: Res<PlayerTextures>,
) {
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        for i in 0..networked_entities.entities.len() {
            if let Some(entity) = network_mapping.0.get(&Entity::from_bits(networked_entities.entities[i])) {
                animate_player(
                    entity,
                    networked_entities.sprite_index[i],
                    networked_entities.sprite_flip_x[i],
                    &networked_entities.texture_entity_type[i],
                    networked_entities.translations[i].into(),
                    &mut commands,
                    &mut texture_atlas_layouts,
                    &player_textures,
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn animate_player(
    entity: &Entity,
    sprite_index:
    usize, sprite_flip_x:
    bool, texture_entity_type: &TextureEntityType,
    translation: Vec3,
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_textures: &Res<PlayerTextures>,
) {
    let (image, layout) = texture_entity_to_handle(
        texture_entity_type,
        texture_atlas_layouts,
        player_textures,
    );

    commands.entity(*entity)
        .insert(Sprite {
            image,
            texture_atlas: Some(TextureAtlas {
                layout,
                index: sprite_index,
            }),
            flip_x: sprite_flip_x,
            ..Default::default()
        })
        .insert(Transform {
            translation,
            scale: Vec3::splat(0.5),
            ..Default::default()
        });
}