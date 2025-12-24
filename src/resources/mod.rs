/// Resources module - Global game state
///
/// Resources are singletons that are accessible to all systems.

pub mod map;
pub mod visibility;
pub mod action_points;
pub mod combat_log;

pub use map::{TileType, CurrentMap};
pub use visibility::{VisibilityState, VisibilityMap};
pub use action_points::PlayerActionPoints;
pub use combat_log::CombatLog;
