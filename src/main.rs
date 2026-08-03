#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use avian3d::prelude::*;
use bevy::prelude::*;
mod player;
use player::PlayerPlugin;
use bevy::window::{PresentMode, CursorGrabMode, PrimaryWindow, CursorOptions};
mod fps;
use fps::FpsPlugin;
mod book;
use bevy_mod_outline::*;
mod interactable;
use interactable::{Interactable, InteractablePlugin};
use book::Book;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin{primary_window: Some(Window{present_mode: PresentMode::AutoNoVsync,..default()}),..default()}), PhysicsPlugins::default(), PlayerPlugin, FpsPlugin, OutlinePlugin::EXTRUDE_VERTEX, InteractablePlugin))
        .add_systems(Startup, (setup_world, setup_cursor))
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut cube_mesh = Cuboid::new(1.0, 1.0, 1.0).mesh().build();
    cube_mesh
        .generate_outline_normals(&GenerateOutlineNormalsSettings::default())
        .unwrap();
    
    let mesh_handle = meshes.add(cube_mesh);

    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.9),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Book {
            text: "My Book".to_string(),
            repeat: false,
        },
        Interactable{},
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