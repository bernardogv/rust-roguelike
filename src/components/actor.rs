/// Actor components - Player and entity markers

use bevy::prelude::*;

/// Marker component for the player entity
#[derive(Component)]
pub struct Player;

/// Component for rendering entities as colored squares
#[derive(Component)]
pub struct Renderable {
    pub color: Color,
}

impl Renderable {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
