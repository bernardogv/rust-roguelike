/// Systems module - Game logic systems
///
/// Systems operate on entities with specific components.

pub mod movement;
pub mod fov;
pub mod turn_manager;
pub mod enemy_ai;
pub mod combat;
pub mod enemy_spawning;

pub use movement::{player_input_system, apply_movement_system, camera_follow_system};
pub use fov::{
    MapTile, TileBaseColor,
    calculate_fov_system,
    update_visibility_map_system,
    apply_tile_visibility_system,
    hide_entities_outside_fov_system,
};
pub use turn_manager::{check_turn_end_system, start_player_turn_system, enemy_turn_system};
pub use enemy_ai::enemy_action_system;
pub use combat::{
    PendingAttack,
    player_attack_input_system,
    execute_attack_system,
    check_player_death_system,
};
pub use enemy_spawning::spawn_enemies_system;
