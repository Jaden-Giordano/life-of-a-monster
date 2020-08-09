use amethyst::{
    assets::{Handle},
    core::{
        math::{Vector3},
        transform::Transform,
    },
    prelude::*,
    renderer::{Camera, SpriteRender, SpriteSheet, Transparent},
    tiles::{FlatEncoder, TileMap},
};

use crate::resources::{Hero, Dungeon, DungeonRenderTile};

pub struct GameplayState {
    pub tileset_handle: Handle<SpriteSheet>,
    pub entity_sprites_handle: Handle<SpriteSheet>,
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        init_camera(world);
        init_map(world, self.tileset_handle.clone());
        init_hero(world, self.entity_sprites_handle.clone());
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(32.0, -32.0, 8.0);

    world
        .create_entity()
        .with(transform)
        .with(Camera::standard_2d(64.0, 64.0))
        .build();
}

fn init_map(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let map = TileMap::<DungeonRenderTile, FlatEncoder>::new(
        Vector3::new(8, 8, 1),
        Vector3::new(8, 8, 1),
        Some(sprite_sheet_handle),
    );
    let mut transform = Transform::default();
    transform.set_translation_xyz(36., -36., 0.);

    world.insert(Dungeon::default());

    // let tiles_per_floor: u32 = 8;
    // let tile_count = dungeon.tiles.len() as u32 / tiles_per_floor;
    // let offset = 3 - tile_count / 8;
    // for (index, tile) in dungeon.tiles.iter().enumerate() {
    //     let x = index as u32 % tiles_per_floor;
    //     let y = offset + (index as u32) / tiles_per_floor;
    // }

    world
        .create_entity()
        .with(map)
        .with(transform)
        .build();
}

fn init_hero(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(4., -4., 1.);
    let sprite_render = SpriteRender {
        sprite_number: 0,
        sprite_sheet: sprite_sheet_handle,
    };

    world
        .create_entity()
        .with(transform)
        .with(sprite_render)
        .with(Transparent)
        .with(Hero::new())
        .build();
}
