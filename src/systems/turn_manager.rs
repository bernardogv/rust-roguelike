/// Turn management system for turn-based gameplay

use bevy::prelude::*;
use crate::resources::PlayerActionPoints;
use crate::states::TurnState;

// ============================================================================
// TURN TRANSITION SYSTEMS
// ============================================================================

/// Check if player's turn should end (runs during PlayerTurn state)
pub fn check_turn_end_system(
    action_points: Res<PlayerActionPoints>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    if action_points.is_depleted() {
        info!("Player action points depleted! Transitioning to EnemyTurn");
        next_state.set(TurnState::EnemyTurn);
    }
}

/// Reset action points at the start of player's turn (runs OnEnter PlayerTurn)
pub fn start_player_turn_system(
    mut action_points: ResMut<PlayerActionPoints>,
) {
    action_points.reset();
    info!("Player turn started! Action points reset to {}/{}",
          action_points.current, action_points.max);
}

/// Handle enemy turn logic (runs during EnemyTurn state)
/// Currently a stub - will be expanded in Phase 5 with enemy AI
pub fn enemy_turn_system(
    mut next_state: ResMut<NextState<TurnState>>,
) {
    // Stub: In Phase 5, this will coordinate enemy actions
    // For now, immediately transition back to PlayerTurn

    info!("Enemy turn complete! Transitioning to PlayerTurn");
    next_state.set(TurnState::PlayerTurn);
}
