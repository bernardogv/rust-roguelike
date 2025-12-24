/// Components module - ECS component definitions
///
/// Components are data attached to entities.

pub mod actor;

pub use actor::{Player, Renderable};
// Re-export Position from resources for convenience
pub use crate::resources::map::Position;
