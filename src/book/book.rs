use bevy::prelude::*;
use crate::interactable::InteractableTrait;

#[derive(Component, Default)]
pub struct Book {
    pub text: String,
    pub repeat: bool,
}

impl InteractableTrait for Book {
    fn interact(&self) {
        // TODO
        println!("Reading book: {}", self.text);
    }
}