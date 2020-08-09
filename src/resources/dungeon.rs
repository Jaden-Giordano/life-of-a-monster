use amethyst::{
    core::math::Vector3,
    tiles::{CoordinateEncoder, FlatEncoder},
};

use crate::resources::DungeonTile;

pub struct Dungeon {
    encoder: FlatEncoder,
    pub tiles: Vec<DungeonTile>,
}

impl Default for Dungeon {
    fn default() -> Dungeon {
        Dungeon {
            encoder: FlatEncoder::from_dimensions(Vector3::new(8, 8, 0)),
            tiles: vec![DungeonTile::Entrance, DungeonTile::Hall, DungeonTile::Zombies, DungeonTile::Hall, DungeonTile::Hall, DungeonTile::Hall, DungeonTile::Hall, DungeonTile::Hall, DungeonTile::End],
        }
    }
}

impl Dungeon {
    pub fn fetch(&self, x: u32, y: u32) -> Option<&DungeonTile> {
        let mut tile: Option<&DungeonTile> = None;
        if let Some(tile_index) = self.encoder.encode(x, y, 0) {
            if (tile_index as usize) < self.tiles.len() {
                tile = Some(&self.tiles[tile_index as usize]);
            }
        }
        tile
    }

    pub fn fetch_mut(&mut self, x: u32, y: u32) -> Option<&mut DungeonTile> {
        let mut tile: Option<&mut DungeonTile> = None;
        if let Some(tile_index) = self.encoder.encode(x, y, 0) {
            if (tile_index as usize) < self.tiles.len() {
                tile = Some(&mut self.tiles[tile_index as usize]);
            }
        }
        tile
    }
}
