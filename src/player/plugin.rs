use bevy::prelude::*;
use super::player::*;
use super::camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup, camera_setup).chain());
        app.add_systems(Update, (player_move, player_rotate, camera_system));
    }
}