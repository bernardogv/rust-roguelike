/// Player movement and camera systems

use bevy::prelude::*;
use crate::components::{Player, Position};
use crate::resources::{CurrentMap, PlayerActionPoints};
use crate::constants::{CAMERA_FOLLOW_SPEED, MOVEMENT_ACTION_COST};

/// Stores pending movement for the player
#[derive(Resource, Default)]
pub struct PendingMovement {
    pub dx: i32,
    pub dy: i32,
}

/// System to handle player input (WASD and arrow keys)
pub fn player_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut pending_movement: ResMut<PendingMovement>,
) {
    pending_movement.dx = 0;
    pending_movement.dy = 0;

    // WASD movement
    if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp) {
        pending_movement.dy = 1;
    }
    if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
        pending_movement.dy = -1;
    }
    if keyboard.just_pressed(KeyCode::KeyA) || keyboard.just_pressed(KeyCode::ArrowLeft) {
        pending_movement.dx = -1;
    }
    if keyboard.just_pressed(KeyCode::KeyD) || keyboard.just_pressed(KeyCode::ArrowRight) {
        pending_movement.dx = 1;
    }
}

/// System to apply movement with collision detection and action point consumption
pub fn apply_movement_system(
    mut query: Query<&mut Position, With<Player>>,
    pending_movement: Res<PendingMovement>,
    map: Res<CurrentMap>,
    mut action_points: ResMut<PlayerActionPoints>,
) {
    // Only move if there's pending movement
    if pending_movement.dx == 0 && pending_movement.dy == 0 {
        return;
    }

    // Check if player can afford the movement
    if !action_points.can_afford(MOVEMENT_ACTION_COST) {
        info!("Not enough action points to move! ({}/{})",
              action_points.current, action_points.max);
        return;
    }

    for mut pos in query.iter_mut() {
        let new_x = pos.x + pending_movement.dx;
        let new_y = pos.y + pending_movement.dy;

        // Check if the new position is walkable
        if map.is_walkable(new_x, new_y) {
            pos.x = new_x;
            pos.y = new_y;

            // Spend action points for successful movement
            action_points.spend(MOVEMENT_ACTION_COST);

            info!("Player moved to ({}, {}) - Action points: {}/{}",
                  pos.x, pos.y, action_points.current, action_points.max);
        } else {
            info!("Blocked! Cannot move to ({}, {})", new_x, new_y);
        }
    }
}

/// System to make camera follow the player
pub fn camera_follow_system(
    player_query: Query<&Position, With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Convert grid position to world position
            let target_x = player_pos.x as f32 * 32.0; // TILE_SIZE = 32
            let target_y = player_pos.y as f32 * 32.0;

            // Smooth camera follow (lerp)
            let target = Vec3::new(target_x, target_y, camera_transform.translation.z);
            camera_transform.translation = camera_transform.translation.lerp(
                target,
                CAMERA_FOLLOW_SPEED * time.delta_secs(),
            );
        }
    }
}
