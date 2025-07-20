use bevy::prelude::{AssetServer, Assets, Bundle, Commands, Component, Name, Res, ResMut, Resource, Sprite, TextureAtlas, TextureAtlasLayout, Timer, TimerMode, Transform, UVec2, Vec3};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Friction, GravityScale, LockedAxes, RigidBody, Sensor, Velocity};
use bevy_renet2::prelude::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

#[derive(Component)]
pub struct Player(pub f32);

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
    pub first: usize,
    pub last: usize,
}
#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub velocity: Velocity,
    pub collider: Collider,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub player: Player,
    pub player_input: PlayerInput,
    pub grounded: Grounded,
    pub jump_counter: JumpCounter,
}

#[derive(Bundle)]
pub struct SensorBundle {
    pub name: Name,
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub transform: Transform,
}

impl Default for SensorBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Sensor"),
            collider: Collider::cuboid(10.0, 2.0),
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
            transform: Transform::from_xyz(0.0, -34.0, 0.0),
        }
    }
}

// only for client standalone
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
    let client_id = "default_client";

    let player_entity = commands.spawn((
        Name::new(format!("Player_{client_id}")),
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

pub fn create_player_bundle(client_id: &ClientId, position: Vec3) -> (PlayerBundle, Transform, SensorBundle) {
    let player_bundle = PlayerBundle {
        name: Name::new(format!("Player_{client_id}")),
        rigid_body: RigidBody::Dynamic,
        locked_axes: LockedAxes::ROTATION_LOCKED,
        velocity: Velocity::zero(),
        collider: Collider::cuboid(16.0, 32.0),
        gravity_scale: GravityScale(1.),
        friction: Friction::coefficient(0.0),
        animation_indices: AnimationIndices { first: 1, last: 3 },
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        player: Player(350.),
        player_input: PlayerInput::default(),
        grounded: Grounded(false),
        jump_counter: JumpCounter { jumps_left: 2, max_jumps: 2 },
    };

    let transform = Transform::from_translation(position);
    let sensor_bundle = SensorBundle {
        transform: Transform::from_translation(Vec3::new(0.0, -34.0, 0.0)),
        ..Default::default()
    };
    (player_bundle, transform, sensor_bundle)
}