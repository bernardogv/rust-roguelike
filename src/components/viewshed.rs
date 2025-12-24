/// Viewshed component for field of view calculations

use bevy::prelude::*;
use std::collections::HashSet;
use crate::resources::map::Position;

/// Component that tracks visible tiles for an entity
#[derive(Component)]
pub struct Viewshed {
    /// Tiles currently visible to this entity
    pub visible_tiles: HashSet<Position>,

    /// Maximum viewing distance
    pub range: i32,

    /// Whether FOV needs recalculation (set on movement)
    pub dirty: bool,
}

impl Viewshed {
    /// Create a new viewshed with the given range
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            range,
            dirty: true, // Start dirty to calculate on first frame
        }
    }

    /// Check if a position is visible to this entity
    pub fn can_see(&self, pos: &Position) -> bool {
        self.visible_tiles.contains(pos)
    }

    /// Mark the viewshed as needing recalculation
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}
