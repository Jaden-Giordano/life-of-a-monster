use amethyst::{
    assets::{Handle},
    core::{
        math::{Point3, Vector3},
        transform::Transform,
    },
    prelude::*,
    renderer::{Camera, SpriteSheet},
    tiles::{MapStorage, TileMap},
};

use crate::resources::SimpleTile;

pub struct GameplayState {
    pub sprite_sheet_handle: Handle<SpriteSheet>,
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        init_camera(world);
        init_map(world, self.sprite_sheet_handle.clone());
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(transform)
        .with(Camera::standard_2d(64.0, 64.0))
        .build();
}

fn init_map(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut map = TileMap::<SimpleTile>::new(
        Vector3::new(8, 8, 1),
        Vector3::new(8, 8, 1),
        Some(sprite_sheet_handle),
    );
    let mut transform = Transform::default();
    transform.set_translation_xyz(4.0, -4.0, 0.0);

    (0..8).for_each(|x| {
        (0..8).for_each(|y| {
            *map.get_mut(&Point3::<u32>::new(x, y, 0)).unwrap() = SimpleTile(Some(0));
        });
    });

    *map.get_mut(&Point3::<u32>::new(2, 3, 0)).unwrap() = SimpleTile(Some(2));
    *map.get_mut(&Point3::<u32>::new(3, 3, 0)).unwrap() = SimpleTile(Some(1));
    *map.get_mut(&Point3::<u32>::new(4, 3, 0)).unwrap() = SimpleTile(Some(1));
    *map.get_mut(&Point3::<u32>::new(5, 3, 0)).unwrap() = SimpleTile(Some(3));

    world
        .create_entity()
        .with(map)
        .with(transform)
        .build();
}
