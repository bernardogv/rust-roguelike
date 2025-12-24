/// Action points resource for turn-based gameplay

use bevy::prelude::*;
use crate::constants::PLAYER_STARTING_ACTION_POINTS;

/// Tracks player's action points for turn-based gameplay
#[derive(Resource)]
pub struct PlayerActionPoints {
    pub current: i32,
    pub max: i32,
}

impl Default for PlayerActionPoints {
    fn default() -> Self {
        Self {
            current: PLAYER_STARTING_ACTION_POINTS,
            max: PLAYER_STARTING_ACTION_POINTS,
        }
    }
}

impl PlayerActionPoints {
    /// Reset action points to maximum (called at start of player turn)
    pub fn reset(&mut self) {
        self.current = self.max;
    }

    /// Check if player can afford an action
    pub fn can_afford(&self, cost: i32) -> bool {
        self.current >= cost
    }

    /// Spend action points
    pub fn spend(&mut self, cost: i32) {
        self.current = (self.current - cost).max(0);
    }

    /// Check if action points are depleted
    pub fn is_depleted(&self) -> bool {
        self.current <= 0
    }
}
