use crate::network::{PlayerMapping, ProjectileMapping};
use bevy::asset::Assets;
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::Vec3;
use bevy::prelude::{Children, Commands, Entity, Quat, Query, Res, ResMut, Sprite, Transform, With};
use bevy::utils::default;
use bevy_renet2::prelude::RenetClient;
use game_core::network::network::{NetworkedEntities, ServerChannel};
use game_core::player::command::handle_from_player_texture;
use game_core::player::component::PlayerNetwork;
use game_core::player::texture::PlayerTextures;
use game_core::weapon::animation::{weapon_rotation, weapon_sprite_flip};
use game_core::weapon::command::{handle_from_weapon_fx_texture, spawn_weapon_fx};
use game_core::weapon::component::{PivotDisk, Weapon};
use game_core::weapon::fx_texture::{FxComponent, WeaponFxTextures};

#[allow(clippy::too_many_arguments)]
pub fn player_animation(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut disk_query: Query<(&mut Transform, &Children), With<PivotDisk>>,
    mut weapon_query: Query<&mut Sprite, With<Weapon>>,
    player_mapping: ResMut<PlayerMapping>,
    mut projectile_mapping: ResMut<ProjectileMapping>,
    player_textures: Res<PlayerTextures>,
    mut weapon_fx_textures: Res<WeaponFxTextures>,
    player_query: Query<&Children, With<PlayerNetwork>>,
    weapon_fx_query: Query<&FxComponent>,
) {
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        for player_entity_index in 0..networked_entities.entities.len() {
            if !networked_entities.entities.is_empty() {
                if let Some(entity) = player_mapping.0.get(&Entity::from_bits(networked_entities.entities[player_entity_index])) {
                    animate_player(
                        entity,
                        &networked_entities,
                        player_entity_index,
                        &mut commands,
                        &mut texture_atlas_layouts,
                        &player_textures,
                    );
                    animate_weapon(
                        entity,
                        &networked_entities,
                        player_entity_index,
                        player_query,
                        &mut disk_query,
                        &mut weapon_query,
                    );
                }
            }
        }

        for _projectile_entity_index in 0..networked_entities.projectile_entities.len() {
            if !networked_entities.projectile_entities.is_empty() {
                animate_weapon_fx(
                    &networked_entities,
                    &mut projectile_mapping,
                    &mut commands,
                    &mut texture_atlas_layouts,
                    &mut weapon_fx_textures,
                    weapon_fx_query,
                );
            }
        }
    }
}
fn animate_player(
    entity: &Entity,
    networked_entities: &NetworkedEntities,
    entity_index: usize,
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_textures: &Res<PlayerTextures>,
) {
    let (image, layout) = handle_from_player_texture(
        &networked_entities.player_texture_entity_type[entity_index],
        texture_atlas_layouts,
        player_textures,
    );

    let translation = networked_entities.translations[entity_index].into();

    commands.entity(*entity)
        .insert(Sprite {
            image,
            texture_atlas: Some(TextureAtlas {
                layout,
                index: networked_entities.sprite_index[entity_index],
            }),
            flip_x: networked_entities.sprite_flip_x[entity_index],
            ..Default::default()
        })
        .insert(Transform {
            translation,
            scale: Vec3::splat(0.5),
            ..Default::default()
        });
}


fn animate_weapon(
    entity: &Entity,
    networked_entities: &NetworkedEntities,
    entity_index: usize,
    player_query: Query<&Children, With<PlayerNetwork>>,
    disk_query: &mut Query<(&mut Transform, &Children), With<PivotDisk>>,
    weapon_query: &mut Query<&mut Sprite, With<Weapon>>,
) {
    if let Ok(children) = player_query.get(*entity) {
        for &child in children.iter() {
            if let Ok((mut transform, weapon_children)) = disk_query.get_mut(child) {
                let aim_direction = networked_entities.player_aim_direction[entity_index];
                weapon_rotation(&mut transform, aim_direction);
                for &weapon_entity in weapon_children.iter() {
                    if let Ok(mut weapon_sprite) = weapon_query.get_mut(weapon_entity) {
                        weapon_sprite_flip(&mut weapon_sprite, aim_direction);
                    }
                }
            }
        }
    }
}

// Rust
fn animate_weapon_fx(
    networked_entities: &NetworkedEntities,
    projectile_mapping: &mut ResMut<ProjectileMapping>,
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    weapon_fx_textures: &mut Res<WeaponFxTextures>,
    weapon_fx_query: Query<&FxComponent>,
) {
    for (i, entity_bits) in networked_entities.projectile_entities.iter().enumerate() {
        let entity = Entity::from_bits(*entity_bits);

        let translation = networked_entities.projectile_translations.get(i);
        let aim_direction = networked_entities.player_aim_direction.get(i);

        if let (Some(translation), Some(aim_direction)) = (translation, aim_direction) {
            if !client_has_entity(entity, projectile_mapping) {
                let client_entity = spawn_weapon_fx(
                    commands,
                    texture_atlas_layouts,
                    weapon_fx_textures,
                    (*translation).into(),
                    &networked_entities.weapon_fx_texture_type,
                    *aim_direction,
                    false,
                );
                projectile_mapping.0.insert(entity, client_entity);
            } else {
                for _ in weapon_fx_query.iter() {
                    if let Some(client_entity) = projectile_mapping.0.get(&entity) {
                        let (image, layout) = handle_from_weapon_fx_texture(
                            &networked_entities.weapon_fx_texture_type,
                            texture_atlas_layouts,
                            weapon_fx_textures,
                        );

                        commands.entity(*client_entity)
                            .insert(Sprite {
                                image,
                                texture_atlas: Some(TextureAtlas {
                                    layout,
                                    index: networked_entities.projectile_sprite_index[i],
                                }),
                                flip_y: networked_entities.projectile_sprite_flip_y[i],
                                ..default()
                            })
                            .insert(Transform {
                                translation: (*translation).into(),
                                scale: Vec3::splat(1.),
                                rotation: Quat::from_rotation_z(networked_entities.player_aim_direction[i]),
                            });
                    }
                }
            }
        }
    }

    despawn_weapon_fx(networked_entities, projectile_mapping, commands);
}

// Rust
fn despawn_weapon_fx(
    networked_entities: &NetworkedEntities,
    projectile_mapping: &mut ResMut<ProjectileMapping>,
    commands: &mut Commands,
) {
    let active_entities: std::collections::HashSet<_> = networked_entities.projectile_entities.iter()
        .map(|bits| Entity::from_bits(*bits))
        .collect();

    let to_despawn: Vec<Entity> = projectile_mapping.0.keys()
        .filter(|e| !active_entities.contains(e))
        .cloned()
        .collect();

    for entity in to_despawn {
        if let Some(client_entity) = projectile_mapping.0.remove(&entity) {
            commands.entity(client_entity).despawn();
        }
    }
}
fn client_has_entity(entity: Entity, network_mapping: &mut ProjectileMapping) -> bool {
    network_mapping.0.contains_key(&entity)
}