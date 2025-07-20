use crate::entities::decor::component::Ground;
use bevy::prelude::{Assets, Color, ColorMaterial, Commands, GlobalTransform, Mesh, Mesh2d, MeshMaterial2d, Rectangle, ResMut, Transform, Vec2};
use bevy_rapier2d::prelude::{Collider, RigidBody};

pub fn setup_ground(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let ground = Ground::new(
        Vec2::new(800.0, 25.0),
        Vec2::new(0.0, -200.0),
    );
    create_ground(commands, meshes, materials, ground);
}

pub fn create_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ground: Ground,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(ground.size.x, ground.size.y))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.5, 0.3)))),
        Transform::from_translation(Vec2::new(ground.position.x, ground.position.y).extend(0.0)),
        RigidBody::Fixed,
        Collider::cuboid(ground.size.x / 2.0, ground.size.y / 2.0),
        GlobalTransform::default(),
        ground,
    ));
}