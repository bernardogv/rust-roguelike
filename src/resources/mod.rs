/// Resources module - Global game state
///
/// Resources are singletons that are accessible to all systems.

pub mod map;
pub mod visibility;

pub use map::{TileType, CurrentMap};
pub use visibility::{VisibilityState, VisibilityMap};
