use avian3d::prelude::*;
use bevy::prelude::*;
mod player;
use player::PlayerPlugin;
use bevy::window::{CursorGrabMode, PrimaryWindow, CursorOptions};
mod fps;
use fps::FpsPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default(), PlayerPlugin, FpsPlugin))
        .add_systems(Startup, (setup_world, setup_cursor))
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, -5.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.5, 0.8),
            ..default()
        })),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_cursor(
    mut commands: Commands,
    windows: Query<Entity, With<PrimaryWindow>>
) {
    let window = windows.single().unwrap();
    commands.entity(window).insert(CursorOptions {
        visible: false,
        grab_mode: CursorGrabMode::Locked,
        hit_test: true,
    });
}