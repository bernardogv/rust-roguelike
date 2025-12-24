/// Resources module - Global game state
///
/// Resources are singletons that are accessible to all systems.

pub mod map;

pub use map::{TileType, CurrentMap};
