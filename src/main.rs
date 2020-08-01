use amethyst::{
    core::{transform::TransformBundle},
    prelude::*,
    renderer::{
        palette::Srgba,
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    tiles::RenderTiles2D,
    utils::application_root_dir,
};

use crate::states::LoadingState;
use crate::resources::SimpleTile;

mod states;
mod resources;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let (r, g, b, a) = Srgba::new(34. / 255., 12. / 255., 39. / 255., 1.0)
        .into_linear()
        .into_components();

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([r, g, b, a]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<SimpleTile>::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets_dir, LoadingState::default(), game_data)?;
    game.run();

    Ok(())
}
