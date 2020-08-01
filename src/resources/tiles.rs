use amethyst::{
    core::math::Point3,
    prelude::*,
    tiles::Tile,
};

#[derive(Clone, Default)]
pub struct SimpleTile(Option<usize>);

impl Tile for SimpleTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        self.0
    }
}
