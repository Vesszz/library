use bevy::prelude::*;
use super::player::Player;

#[derive(Component, Default)]
pub struct PlayerCamera();

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        PlayerCamera::default()
    ));
}

pub fn camera_system(
    player_query: Query<(&Transform, &Player), Without<PlayerCamera>>,
    mut player_camera_query: Query<&mut Transform, With<PlayerCamera>>
) {
    let (player_transform, player) = player_query.single().unwrap();
    let mut player_camera_transform = player_camera_query.single_mut().unwrap();
    player_camera_transform.translation = player_transform.translation; // + delta height
    let target = player_camera_transform.translation + player.look_direction;
    player_camera_transform.look_at(target, Vec3::Y);
}
