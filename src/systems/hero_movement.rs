use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    prelude::*,
    shred::{ResourceId, SystemData},
};
use amethyst::derive::SystemDesc;

use crate::resources::Hero;

#[derive(SystemDesc)]
pub struct HeroMovementSystem;

#[derive(SystemData)]
pub struct HeroMovementSystemData<'a> {
    pub transforms: WriteStorage<'a, Transform>,
    pub heroes: ReadStorage<'a, Hero>,
    pub time: Read<'a, Time>,
}

impl<'s> System<'s> for HeroMovementSystem {
    type SystemData = HeroMovementSystemData<'s>;

    fn run(
        &mut self,
        HeroMovementSystemData {
            mut transforms,
            heroes,
            time,
        }: Self::SystemData
    ) {
        for (transform, hero) in (&mut transforms, &heroes).join() {
            transform.prepend_translation_x(hero.speed * time.delta_seconds());
        }
    }
}
