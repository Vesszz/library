use bevy::prelude::*;
use super::fps::{fps_setup, fps_update};

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fps_setup);
        app.add_systems(Update, fps_update);
    }
}