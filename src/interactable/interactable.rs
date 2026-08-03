use bevy::prelude::*;
use bevy_mod_outline::*;

#[derive(Component)]
pub struct Interactable {}

pub fn auto_outline(
    trigger: On<Add, Interactable>,
    mut commands: Commands,
) {
    let entity = trigger.entity;
    commands.entity(entity).insert(OutlineVolume {
                    visible: true,
                    colour: Color::srgb(1.0, 1.0, 1.0),
                    width: 25.0,
    },);
}

pub trait InteractableTrait {
    fn interact(&self);
}