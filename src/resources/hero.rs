use amethyst::{
    core::math::Point3,
    ecs::{
        Component,
        DenseVecStorage,
    },
};

#[derive(Debug)]
pub enum HeroState {
    Idle,
    Progressing(Point3<u32>),
    Fighting(Point3<u32>),
    Halted(Point3<u32>),
    Dead(Point3<u32>),
}

pub struct Hero {
    pub state: HeroState,
    pub health: i32,
    pub speed: f32,
}

impl Hero {
    pub fn new() -> Hero {
        Hero {
            state: HeroState::Idle,
            health: 15,
            speed: 2.0,
        }
    }
}

impl Component for Hero {
    type Storage = DenseVecStorage<Self>;
}
