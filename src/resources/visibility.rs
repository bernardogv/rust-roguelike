/// Visibility state management for fog of war

use bevy::prelude::*;
use std::collections::HashMap;
use crate::resources::map::Position;

/// The visibility state of a tile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityState {
    /// Never seen by the player
    Unseen,

    /// Seen before but not currently visible
    Explored,

    /// Currently visible in FOV
    Visible,
}

/// Global resource tracking which tiles have been explored
#[derive(Resource)]
pub struct VisibilityMap {
    /// Map of positions to their visibility state
    pub tiles: HashMap<Position, VisibilityState>,
}

impl VisibilityMap {
    /// Create a new empty visibility map
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    /// Get the visibility state of a tile (defaults to Unseen)
    pub fn get(&self, pos: &Position) -> VisibilityState {
        self.tiles.get(pos).copied().unwrap_or(VisibilityState::Unseen)
    }

    /// Set a tile's visibility state
    pub fn set(&mut self, pos: Position, state: VisibilityState) {
        self.tiles.insert(pos, state);
    }

    /// Mark a tile as explored (preserves Visible state)
    pub fn mark_explored(&mut self, pos: Position) {
        self.tiles.entry(pos)
            .and_modify(|state| {
                if *state != VisibilityState::Visible {
                    *state = VisibilityState::Explored;
                }
            })
            .or_insert(VisibilityState::Explored);
    }

    /// Clear all visibility (reset fog of war)
    pub fn clear(&mut self) {
        self.tiles.clear();
    }
}

impl Default for VisibilityMap {
    fn default() -> Self {
        Self::new()
    }
}
