use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, System, Write, WriteStorage},
    prelude::*,
    shred::{ResourceId, SystemData},
};
use amethyst::derive::SystemDesc;

use crate::resources::{Dungeon, Hero, HeroState};

const DELAY: f32 = 2.5;

#[derive(SystemData)]
pub struct FightingSystemData<'a> {
    heroes: WriteStorage<'a, Hero>,
    dungeon: Write<'a, Dungeon>,
    time: Read<'a, Time>,
}

#[derive(SystemDesc, Default)]
pub struct FightingSystem {
    timer: f32,
}

impl<'s> System<'s> for FightingSystem {
    type SystemData = FightingSystemData<'s>;

    fn run(
        &mut self,
        FightingSystemData {
            mut heroes,
            mut dungeon,
            time,
        }: Self::SystemData,
    ) {
        for hero in (&mut heroes).join() {
            if let HeroState::Fighting(tile_position) = hero.state {
                if self.timer >= DELAY {
                    self.timer = 0.0;
                    if let Some(tile) = dungeon.fetch_mut(tile_position.x, tile_position.y) {
                        if !tile.complete {
                            hero.health -= tile.damage;
                            tile.complete = true;
                            println!("OUCH! Hero took {} points of damage; now at {} hp.", tile.damage, hero.health);
                            hero.state = HeroState::Progressing(tile_position);
                        }
                    }
                } else {
                    self.timer += time.delta_seconds();
                }
            }
        }
    }
}
