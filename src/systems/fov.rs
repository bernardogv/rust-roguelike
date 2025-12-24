/// Field of view calculation and visibility systems

use bevy::prelude::*;
use bracket_pathfinding::prelude::*;
use crate::components::{Player, Position, Viewshed};
use crate::resources::{CurrentMap, VisibilityMap, VisibilityState};

// ============================================================================
// COMPONENTS
// ============================================================================

/// Marker component for map tiles
#[derive(Component)]
pub struct MapTile {
    pub position: Position,
}

/// Stores the original color of a tile for visibility calculations
#[derive(Component)]
pub struct TileBaseColor(pub Color);

// ============================================================================
// SYSTEMS
// ============================================================================

/// Calculate FOV for entities that have moved
pub fn calculate_fov_system(
    mut query: Query<(&Position, &mut Viewshed), Changed<Position>>,
    map: Res<CurrentMap>,
) {
    for (pos, mut viewshed) in query.iter_mut() {
        if !viewshed.dirty {
            continue;
        }

        // Convert Position to bracket-lib Point
        let origin = Point::new(pos.x, pos.y);

        // Use bracket-pathfinding's symmetric shadowcasting
        let visible = field_of_view_set(
            origin,
            viewshed.range,
            &*map  // Deref to access BaseMap implementation
        );

        // Convert HashSet<Point> to HashSet<Position>
        viewshed.visible_tiles = visible
            .iter()
            .map(|pt| Position::new(pt.x, pt.y))
            .collect();

        viewshed.dirty = false;

        info!("FOV calculated at ({}, {}): {} tiles visible",
              pos.x, pos.y, viewshed.visible_tiles.len());
    }
}

/// Update global visibility map based on player's viewshed
pub fn update_visibility_map_system(
    query: Query<&Viewshed, (With<Player>, Changed<Viewshed>)>,
    mut visibility_map: ResMut<VisibilityMap>,
) {
    // Only process if player's viewshed changed
    if let Ok(viewshed) = query.get_single() {
        // First, downgrade all Visible tiles to Explored
        for (_pos, state) in visibility_map.tiles.iter_mut() {
            if *state == VisibilityState::Visible {
                *state = VisibilityState::Explored;
            }
        }

        // Then mark currently visible tiles
        for pos in &viewshed.visible_tiles {
            visibility_map.set(*pos, VisibilityState::Visible);
        }

        info!("Visibility map updated: {} explored tiles",
              visibility_map.tiles.len());
    }
}

/// Update tile colors based on visibility state
pub fn apply_tile_visibility_system(
    visibility_map: Res<VisibilityMap>,
    mut query: Query<(&MapTile, &TileBaseColor, &mut Sprite)>,
) {
    // Only run when visibility map changes
    if !visibility_map.is_changed() {
        return;
    }

    for (tile, base_color, mut sprite) in query.iter_mut() {
        let visibility = visibility_map.get(&tile.position);

        match visibility {
            VisibilityState::Visible => {
                // Full original color
                sprite.color = base_color.0;
            }
            VisibilityState::Explored => {
                // Multiply color by 0.5 for dimmed effect
                let c = base_color.0.to_srgba();
                sprite.color = Color::srgb(
                    c.red * 0.5,
                    c.green * 0.5,
                    c.blue * 0.5,
                );
            }
            VisibilityState::Unseen => {
                // Completely black
                sprite.color = Color::srgb(0.0, 0.0, 0.0);
            }
        }
    }
}

/// Hide entities (enemies, items) outside player's FOV
pub fn hide_entities_outside_fov_system(
    player_query: Query<&Viewshed, (With<Player>, Changed<Viewshed>)>,
    mut entity_query: Query<
        (&Position, &mut Visibility),
        (Without<Player>, Without<MapTile>)
    >,
) {
    // Only process if player viewshed changed
    if let Ok(viewshed) = player_query.get_single() {
        for (pos, mut visibility) in entity_query.iter_mut() {
            if viewshed.can_see(pos) {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
