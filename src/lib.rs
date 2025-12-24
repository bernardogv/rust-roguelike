/// Library exports for the roguelike game
///
/// This module re-exports all public APIs for external use
/// and provides a modular structure for the game components.

pub mod constants;
pub mod states;
pub mod resources;
pub mod components;
pub mod systems;
pub mod plugins;

// Re-export commonly used items
pub use constants::*;
pub use states::{GameState, TurnState, StatesPlugin};
pub use plugins::GameCorePlugin;
