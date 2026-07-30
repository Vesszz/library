use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;

#[derive(Component, Default)]
pub struct Player {
    pub look_direction: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

pub fn player_setup(mut commands: Commands) {
    commands.spawn((
        Player {
            look_direction: Vec3::NEG_Z,
            yaw: 0.0,
            pitch: 0.0,
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
    let (mut player_transform, player) = player_query.single_mut().unwrap();
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

pub fn player_rotate(
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut().unwrap();
    let delta = mouse_motion.delta;
    if delta == Vec2::ZERO {return}
    let sensitivity = 0.05;
    player.yaw += delta.x * sensitivity * time.delta_secs();
    player.pitch -= delta.y * sensitivity * time.delta_secs();
    player.pitch = player.pitch.clamp(-1.5, 1.5);
    player.look_direction = Vec3::new(
        player.yaw.cos() * player.pitch.cos(),
        player.pitch.sin(),
        player.yaw.sin() * player.pitch.cos()
    ).normalize();
}