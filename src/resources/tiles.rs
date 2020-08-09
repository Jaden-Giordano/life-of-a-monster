use amethyst::{
    core::math::Point3,
    prelude::*,
    renderer::palette::Srgba,
    tiles::Tile,
};

use crate::resources::{Dungeon, HeroState};

pub const TILES_PER_FLOOR: u32 = 8;

#[derive(Clone, Default, Debug)]
pub struct DungeonTile {
    pub complete: bool,
    pub damage: i32,
    pub sprite: Option<usize>,
}

impl DungeonTile {
    pub const Blank: Self = create_tile().with_sprite(0).build();
    pub const Hall: Self = create_tile().with_sprite(1).build();
    pub const Entrance: Self = create_tile().with_sprite(2).build();
    pub const End: Self = create_tile().with_sprite(3).build();
    pub const Zombies: Self = create_tile().with_sprite(4).with_damage(10).build();
    pub const Jello: Self = create_tile().with_sprite(5).with_damage(50).build();

    pub fn into_state(&self, tile_position: Point3<u32>) -> HeroState {
        let mut state = HeroState::Idle;
        if self.complete || self.damage == 0 {
            state = HeroState::Progressing(tile_position);
        } else if self.damage > 0 {
            state = HeroState::Fighting(tile_position);
        }
        state
    }
}

#[derive(Clone, Default)]
pub struct DungeonRenderTile;

impl Tile for DungeonRenderTile {
    fn sprite(&self, coords: Point3<u32>, world: &World) -> Option<usize> {
        let mut sprite: Option<usize> = None;
        let dungeon = world.fetch::<Dungeon>();
        if let Some(tile) = dungeon.fetch(coords.x, coords.y) {
            sprite = tile.sprite;
        }
        sprite
    }

    fn tint(&self, coords: Point3<u32>, world: &World) -> Srgba {
        let mut tint = Srgba::new(1.0, 1.0, 1.0, 1.0);
        let dungeon = world.fetch::<Dungeon>();
        if let Some(tile) = dungeon.fetch(coords.x, coords.y) {
            if tile.complete { tint = Srgba::new(0.2, 0.2, 0.2, 0.2); }
        }
        tint
    }
}

#[derive(Default)]
struct TileBuilder {
    sprite: Option<usize>,
    damage: Option<i32>,
}

const fn create_tile() -> TileBuilder {
    TileBuilder {
        sprite: None,
        damage: None,
    }
}

impl TileBuilder {
    const fn build(&self) -> DungeonTile {
        let damage = if let Some(damage) = self.damage { damage } else { 0 };
        DungeonTile {
            complete: false,
            damage,
            sprite: self.sprite,
        }
    }

    const fn with_sprite(self, sprite: usize) -> Self {
        TileBuilder {
            sprite: Some(sprite),
            damage: self.damage,
        }
    }

    const fn with_damage(self, damage: i32) -> Self {
        TileBuilder {
            sprite: self.sprite,
            damage: Some(damage),
        }
    }
}
