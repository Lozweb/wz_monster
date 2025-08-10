use crate::network::NetworkMapping;
use bevy::asset::Assets;
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::Vec3;
use bevy::prelude::{Children, Commands, Entity, Query, Res, ResMut, Sprite, Transform, With};
use bevy_renet2::prelude::RenetClient;
use game_core::network::network::{NetworkedEntities, ServerChannel};
use game_core::player::command::handle_from_player_texture;
use game_core::player::component::PlayerNetwork;
use game_core::player::texture::PlayerTextures;
use game_core::weapon::animation::{weapon_rotation, weapon_sprite_flip};
use game_core::weapon::component::{PivotDisk, Weapon};
#[allow(clippy::too_many_arguments)]
pub fn player_animation(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut disk_query: Query<(&mut Transform, &Children), With<PivotDisk>>,
    mut weapon_query: Query<&mut Sprite, With<Weapon>>,
    network_mapping: ResMut<NetworkMapping>,
    player_textures: Res<PlayerTextures>,
    player_query: Query<&Children, With<PlayerNetwork>>,
) {
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        for entity_index in 0..networked_entities.entities.len() {
            if let Some(entity) = network_mapping.0.get(&Entity::from_bits(networked_entities.entities[entity_index])) {
                animate_player(
                    entity,
                    &networked_entities,
                    entity_index,
                    &mut commands,
                    &mut texture_atlas_layouts,
                    &player_textures,
                );
                animate_weapon(
                    entity,
                    &networked_entities,
                    entity_index,
                    player_query,
                    &mut disk_query,
                    &mut weapon_query,
                );

                // todo animate weapon_fx
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