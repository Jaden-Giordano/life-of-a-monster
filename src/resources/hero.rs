use amethyst::ecs::{
    Component,
    DenseVecStorage,
};

pub enum HeroState {
    Progressing,
    Fighting,
    Halted,
    Dead,
}

pub struct Hero {
    pub state: HeroState,
    pub health: i32,
    pub speed: f32,
}

impl Hero {
    pub fn new() -> Hero {
        Hero {
            state: HeroState::Progressing,
            health: 10,
            speed: 2.0,
        }
    }
}

impl Component for Hero {
    type Storage = DenseVecStorage<Self>;
}
