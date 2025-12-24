/// Components module - ECS component definitions
///
/// Components are data attached to entities.

pub mod actor;
pub mod viewshed;
pub mod combat;

pub use actor::{Player, Renderable};
pub use viewshed::Viewshed;
pub use combat::{Health, CombatStats, Enemy, Name};
// Re-export Position from resources for convenience
pub use crate::resources::map::Position;
