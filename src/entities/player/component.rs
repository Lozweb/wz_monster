use crate::entities::player::player_input::PlayerInput;
use bevy::asset::{AssetServer, Assets};
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::UVec2;
use bevy::prelude::{Commands, Component, Name, Res, ResMut, Sprite, Timer, TimerMode, Transform};
use bevy_rapier2d::dynamics::{GravityScale, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{ActiveEvents, Collider, Friction, Sensor};

#[derive(Component)]
pub struct Player(pub(crate) f32);

#[derive(Component, Default)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct JumpCounter {
    pub jumps_left: u8,
    pub max_jumps: u8,
}
impl JumpCounter {
    pub fn reset(&mut self) {
        self.jumps_left = self.max_jumps;
    }

    pub fn use_jump(&mut self) -> bool {
        if self.jumps_left > 0 {
            self.jumps_left -= 1;
            true
        } else {
            false
        }
    }
}
#[derive(Component)]
pub struct AnimationIndices {
    pub(crate) first: usize,
    pub(crate) last: usize,
}
#[derive(Component)]
pub struct AnimationTimer(pub(crate) Timer);

pub fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/player1.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 64), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 3 };

    let player_entity = commands.spawn((
        Name::new("Player"),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        Collider::cuboid(16.0, 32.0),
        GravityScale(1.),
        Friction::coefficient(0.0),
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
    )).id();

    commands.entity(player_entity).with_children(|parent| {
        parent.spawn((
            Name::new("Player Sensor"),
            Collider::cuboid(10.0, 2.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Transform::from_xyz(0.0, -34.0, 0.0),
        ));
    });
}
