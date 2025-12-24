/// Combat-related components for health, stats, and entity identification

use bevy::prelude::*;

// ============================================================================
// HEALTH COMPONENT
// ============================================================================

/// Entity health tracking
#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self {
            current: max,
            max,
        }
    }

    /// Apply damage and return true if entity died
    pub fn take_damage(&mut self, amount: i32) -> bool {
        self.current = (self.current - amount).max(0);
        self.is_dead()
    }

    /// Check if entity is dead
    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }

    /// Heal entity by amount (capped at max health)
    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Get health as percentage (0.0 to 1.0)
    pub fn percentage(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}

// ============================================================================
// COMBAT STATS COMPONENT
// ============================================================================

/// Combat statistics for attack and defense
#[derive(Component, Debug, Clone, Copy)]
pub struct CombatStats {
    pub power: i32,
    pub defense: i32,
}

impl CombatStats {
    pub fn new(power: i32, defense: i32) -> Self {
        Self { power, defense }
    }
}

// ============================================================================
// ENEMY MARKER COMPONENT
// ============================================================================

/// Marker component for enemy entities
#[derive(Component, Debug)]
pub struct Enemy;

// ============================================================================
// NAME COMPONENT
// ============================================================================

/// Name component for combat log messages
#[derive(Component, Debug, Clone)]
pub struct Name(pub String);

impl Name {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}
