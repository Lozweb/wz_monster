mod system;
mod component;
mod player_input;

use crate::entities::player::player_input::{handle_player_input, PlayerInput};
use bevy::prelude::*;
use component::*;
use system::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, create_player)
            .add_systems(FixedUpdate, (
                player_movement,
                handle_player_input,
                update_grounded_system,
                animate_sprite,
            ));

        app.insert_resource(PlayerInput::default());
    }
}




