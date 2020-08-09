use amethyst::{
    core::{
        math::Point3,
        timing::Time,
        Transform,
    },
    ecs::{Join, Read, System, Write, WriteStorage},
    prelude::*,
    shred::{ResourceId, SystemData},
};
use amethyst::derive::SystemDesc;

use crate::resources::{Dungeon, DungeonTile, Hero, HeroState};

#[derive(SystemDesc)]
pub struct ProgressionSystem;

#[derive(SystemData)]
pub struct ProgressionSystemData<'a> {
    pub transforms: WriteStorage<'a, Transform>,
    pub heroes: WriteStorage<'a, Hero>,
    pub dungeon: Read<'a, Dungeon>,
    pub time: Read<'a, Time>,
}

impl<'s> System<'s> for ProgressionSystem {
    type SystemData = ProgressionSystemData<'s>;

    fn run(
        &mut self,
        ProgressionSystemData {
            mut transforms,
            mut heroes,
            dungeon,
            time,
        }: Self::SystemData
    ) {
        for (transform, hero) in (&mut transforms, &mut heroes).join() {
            let x = transform.translation().x as u32 / 8;
            let y = (transform.translation().y as u32 / 8);
            let cur_tile_position = Point3::<u32>::new(x, y, 0);
            if let HeroState::Progressing(old_tile_position) = hero.state {
                if old_tile_position == cur_tile_position {
                    transform.prepend_translation_x(hero.speed * time.delta_seconds());
                } else if let Some(tile) = dungeon.fetch(x, y).as_ref() {
                    hero.state = tile.into_state(cur_tile_position);
                } else if x >= 8 {
                    transform.set_translation_x(0.0);
                    transform.prepend_translation_y(-8.0);
                }
            } else if let HeroState::Idle = hero.state {
                if let Some(tile) = dungeon.fetch(x, y).as_ref() {
                    if tile.sprite == DungeonTile::Entrance.sprite {
                        hero.state = HeroState::Progressing(cur_tile_position);
                    }
                }
            }
        }
    }
}
