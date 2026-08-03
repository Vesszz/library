use bevy::prelude::*;
use super::interactable::*;

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(auto_outline);
    }
}