/// Combat systems for player attacks, damage calculation, and death handling

use bevy::prelude::*;
use crate::components::{Player, Position, Viewshed, Health, CombatStats, Enemy, Name};
use crate::resources::{PlayerActionPoints, CombatLog};
use crate::states::GameState;
use crate::constants::{ATTACK_ACTION_COST, BASE_HIT_CHANCE, DAMAGE_VARIANCE};

// ============================================================================
// RESOURCES
// ============================================================================

/// Pending attack target (mirrors PendingMovement pattern)
#[derive(Resource, Default)]
pub struct PendingAttack {
    pub target: Option<Entity>,
}

// ============================================================================
// INPUT SYSTEM
// ============================================================================

/// Capture Space bar input and queue attack on adjacent visible enemy
pub fn player_attack_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut pending_attack: ResMut<PendingAttack>,
    action_points: Res<PlayerActionPoints>,
    player_query: Query<(&Position, &Viewshed), With<Player>>,
    enemy_query: Query<(Entity, &Position, &Name), With<Enemy>>,
) {
    // Clear previous pending attack
    pending_attack.target = None;

    // Only process if Space bar pressed
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    // Check if player can afford attack
    if !action_points.can_afford(ATTACK_ACTION_COST) {
        info!("Not enough action points to attack! ({}/{})",
              action_points.current, action_points.max);
        return;
    }

    // Get player position and viewshed
    let (player_pos, player_viewshed) = match player_query.get_single() {
        Ok(data) => data,
        Err(_) => return,
    };

    // Find adjacent enemy in FOV
    for (enemy_entity, enemy_pos, enemy_name) in enemy_query.iter() {
        // Check adjacency (including diagonals)
        let dx = (player_pos.x - enemy_pos.x).abs();
        let dy = (player_pos.y - enemy_pos.y).abs();
        let is_adjacent = dx <= 1 && dy <= 1 && !(dx == 0 && dy == 0);

        if !is_adjacent {
            continue;
        }

        // Check visibility
        if !player_viewshed.can_see(enemy_pos) {
            continue;
        }

        // Found valid target!
        pending_attack.target = Some(enemy_entity);
        info!("Attack queued: {} at ({}, {})", enemy_name.0, enemy_pos.x, enemy_pos.y);
        return;
    }

    info!("No adjacent enemies to attack!");
}

// ============================================================================
// COMBAT EXECUTION SYSTEM
// ============================================================================

/// Execute pending attack: roll hit, calculate damage, apply to health
pub fn execute_attack_system(
    mut commands: Commands,
    mut pending_attack: ResMut<PendingAttack>,
    mut action_points: ResMut<PlayerActionPoints>,
    mut combat_log: ResMut<CombatLog>,
    player_query: Query<(&CombatStats, &Name), With<Player>>,
    mut enemy_query: Query<(&mut Health, &CombatStats, &Name), With<Enemy>>,
) {
    // Check if there's a pending attack
    let target = match pending_attack.target.take() {
        Some(t) => t,
        None => return,
    };

    // Get attacker stats (player)
    let (attacker_stats, attacker_name) = match player_query.get_single() {
        Ok(data) => data,
        Err(_) => return,
    };

    // Get defender stats (enemy)
    let (mut defender_health, defender_stats, defender_name) = match enemy_query.get_mut(target) {
        Ok(data) => data,
        Err(_) => {
            info!("Target enemy no longer exists!");
            return;
        }
    };

    // Resolve combat
    let (hit, damage_dealt) = resolve_combat(attacker_stats, defender_stats, &mut defender_health);

    if hit {
        // Log hit message
        let message = format!(
            "{} hits {} for {} damage! ({}/{} HP)",
            attacker_name.0,
            defender_name.0,
            damage_dealt,
            defender_health.current,
            defender_health.max
        );
        combat_log.add_message(message);

        // Check if enemy died
        if defender_health.is_dead() {
            let death_message = format!("{} dies!", defender_name.0);
            combat_log.add_message(death_message);

            // Despawn enemy
            commands.entity(target).despawn();
        }
    } else {
        // Log miss message
        let message = format!("{} misses {}!", attacker_name.0, defender_name.0);
        combat_log.add_message(message);
    }

    // Spend action point
    action_points.spend(ATTACK_ACTION_COST);
}

// ============================================================================
// COMBAT RESOLUTION
// ============================================================================

/// Roll for hit and calculate damage
fn resolve_combat(
    attacker_stats: &CombatStats,
    defender_stats: &CombatStats,
    defender_health: &mut Health,
) -> (bool, i32) {
    // Roll for hit (0-99 vs BASE_HIT_CHANCE)
    let roll = rand::random::<u32>() % 100;
    let hit = roll < BASE_HIT_CHANCE;

    if !hit {
        return (false, 0);
    }

    // Calculate base damage (power - defense)
    let base_damage = attacker_stats.power - defender_stats.defense;

    // Add variance (±DAMAGE_VARIANCE)
    let variance = (rand::random::<i32>() % (DAMAGE_VARIANCE * 2 + 1)) - DAMAGE_VARIANCE;
    let final_damage = (base_damage + variance).max(1); // Minimum 1 damage

    // Apply damage
    defender_health.take_damage(final_damage);

    (true, final_damage)
}

// ============================================================================
// DEATH HANDLING SYSTEM
// ============================================================================

/// Check if player died and transition to GameOver
pub fn check_player_death_system(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(health) = player_query.get_single() {
        if health.is_dead() {
            info!("Player has died! Game Over!");
            next_state.set(GameState::GameOver);
        }
    }
}
