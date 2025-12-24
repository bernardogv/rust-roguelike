/// Enemy spawning system

use bevy::prelude::*;
use crate::components::{Enemy, Position, Name, Health, CombatStats, Renderable, Viewshed};
use crate::resources::CurrentMap;
use crate::constants::*;

// ============================================================================
// ENEMY SPAWNING
// ============================================================================

/// Spawn 3-5 enemies at random walkable positions
pub fn spawn_enemies_system(
    mut commands: Commands,
    map: &CurrentMap,
) {
    // Random count between ENEMY_MIN_COUNT and ENEMY_MAX_COUNT
    let count = (rand::random::<usize>() % (ENEMY_MAX_COUNT - ENEMY_MIN_COUNT + 1)) + ENEMY_MIN_COUNT;

    info!("Spawning {} enemies", count);

    for i in 0..count {
        let mut attempts = 0;

        loop {
            // Random position
            let x = rand::random::<usize>() % map.width;
            let y = rand::random::<usize>() % map.height;

            // Check: walkable, not player spawn position (10, 10)
            if map.is_walkable(x as i32, y as i32) && (x as i32 != 10 || y as i32 != 10) {
                // Spawn enemy
                commands.spawn((
                    Enemy,
                    Position::new(x as i32, y as i32),
                    Name::new(format!("Goblin #{}", i + 1)),
                    Health::new(ENEMY_STARTING_HEALTH),
                    CombatStats::new(ENEMY_ATTACK_POWER, ENEMY_DEFENSE),
                    Renderable::new(COLOR_ENEMY),
                    Viewshed::new(ENEMY_FOV_RADIUS),
                    Sprite {
                        color: COLOR_ENEMY,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    Transform::from_xyz(
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        Z_LAYER_CHARACTERS,
                    ),
                    Visibility::Hidden, // FOV system will reveal
                ));

                info!("Spawned Goblin #{} at ({}, {})", i + 1, x, y);
                break;
            }

            attempts += 1;
            if attempts > 100 {
                warn!("Failed to find spawn position for enemy {} after 100 attempts", i);
                break;
            }
        }
    }
}
