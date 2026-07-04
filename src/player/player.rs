use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub look_direction: Vec3
}

pub fn player_setup(mut commands: Commands) {
    commands.spawn((
        Player {
            look_direction: Vec3::NEG_Z
        },
        Transform::default(),
        GlobalTransform::default()
    ));
}

pub fn player_move(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>
) {
    let (mut player_transform, mut player) = player_query.single_mut().unwrap();
    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if direction != Vec3::ZERO {
        direction = direction.normalize();
        
        let look_direction = player.look_direction;
        let forward = Vec3::new(look_direction.x, 0.0, look_direction.z).normalize();
        let right = Vec3::new(-look_direction.z, 0.0, look_direction.x).normalize();
        
        let world_direction = forward * direction.z + right * direction.x;
        
        let speed = 1.0;
        player_transform.translation += world_direction * speed * time.delta_secs();
    }
 
}

use bevy::input::mouse::MouseMotion;
pub fn player_rotate(
    mut mouse_motion: EventReader<MouseMotion>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut().unwrap();
    // TODO
}