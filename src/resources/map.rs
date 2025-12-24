/// Map data structures and tile management

use bevy::prelude::*;
use std::collections::HashMap;

/// Types of tiles in the game world
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
}

impl TileType {
    /// Check if this tile can be walked on
    pub fn is_walkable(&self) -> bool {
        matches!(self, TileType::Floor)
    }
}

/// Position component for grid-based entities
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// The current game map
#[derive(Resource)]
pub struct CurrentMap {
    pub tiles: Vec<Vec<TileType>>,
    pub width: usize,
    pub height: usize,
    pub entities_at: HashMap<Position, Vec<Entity>>,
}

impl CurrentMap {
    /// Create a new empty map filled with walls
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![TileType::Wall; width]; height];
        Self {
            tiles,
            width,
            height,
            entities_at: HashMap::new(),
        }
    }

    /// Create a test map with a simple room
    pub fn test_map() -> Self {
        let mut map = Self::new(20, 20);

        // Create a simple room (border walls, floor inside)
        for y in 0..map.height {
            for x in 0..map.width {
                if x == 0 || x == map.width - 1 || y == 0 || y == map.height - 1 {
                    map.tiles[y][x] = TileType::Wall;
                } else {
                    map.tiles[y][x] = TileType::Floor;
                }
            }
        }

        // Add some interior walls for testing collision
        for x in 5..15 {
            map.tiles[10][x] = TileType::Wall;
        }

        map
    }

    /// Get the tile type at a position
    pub fn get_tile(&self, x: i32, y: i32) -> Option<TileType> {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        if y >= self.height || x >= self.width {
            return None;
        }
        Some(self.tiles[y][x])
    }

    /// Check if a position is walkable
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y)
            .map(|t| t.is_walkable())
            .unwrap_or(false)
    }

    /// Check if there's a blocking entity at this position
    pub fn is_blocked(&self, pos: &Position) -> bool {
        !self.is_walkable(pos.x, pos.y)
    }
}
