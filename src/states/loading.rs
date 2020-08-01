use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{formats::texture::ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::states::GameplayState;

#[derive(Default)]
pub struct LoadingState {
    progress_counter: Option<ProgressCounter>,
    tileset_handle: Option<Handle<SpriteSheet>>,
    entity_sprites_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.progress_counter = Some(ProgressCounter::new());
        let world = data.world;
        self.tileset_handle = Some(load_sprite_sheet("texture/tiles", world, &mut self.progress_counter));
        self.entity_sprites_handle = Some(load_sprite_sheet("texture/entities", world, &mut self.progress_counter));
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.as_ref().unwrap().is_complete() {
            Trans::Switch(Box::new(GameplayState {
                tileset_handle: self.tileset_handle.take().expect(
                    "Expected `tileset_handle` to exist when `progress_counter` is complete",
                ),
                entity_sprites_handle: self.entity_sprites_handle.take().expect(
                    "Expected `entity_sprite_handle` to exist when `progress_counter` is complete",
                ),
            }))
        } else {
            Trans::None
        }
    }
}

fn load_sprite_sheet(name: &str, world: &World, progress: &mut Option<ProgressCounter>) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}.png", name),
            ImageFormat::default(),
            progress.as_mut().unwrap(),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}.ron", name),
        SpriteSheetFormat(texture_handle),
        progress.as_mut().unwrap(),
        &sprite_sheet_storage,
    )
}
