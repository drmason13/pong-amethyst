//use std::path::Path;

use amethyst::{
    prelude::*,
    //config::Config,
    input::{InputBundle, StringBindings},
    core::transform::TransformBundle,
    ui::{RenderUi, UiBundle},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod systems;
mod pong;
use crate::pong::Pong;

//mod config;
//use crate::config::PongConfig;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let binding_path = app_root.join("config").join("bindings.ron");
    let display_config_path = app_root.join("config").join("display.ron");
    //let pong_config_path = app_root.join("config").join("arena.ron");
    //let pong_config = PongConfig::load(&pong_config_path)?;
    
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([122.0/255.0, 31.0/255.0, 171.0/255.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
        )?
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        // handle user input
        .with_bundle(input_bundle)?
        // UI for scoreboard
        .with_bundle(UiBundle::<StringBindings>::new())?
        // Add our own custom systems for game logic
        // "paddle_system" controls the paddles, moving them according to user input. It depends on the input system from the InputBundle.
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        // "move_balls_system" moves the ball(s) independently according to their velocity.
        .with(systems::MoveBallsSystem, "ball_system", &[])
        // collision_system bounces balls off the top, bottom and paddles, it needs to run after the paddle and ball systems.
        .with(
            systems::CollisionSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        )
        .with(systems::WinnerSystem, "winner_system", &["ball_system"])
        .with(systems::FreezeSystem, "freeze_system", &["winner_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();

    Ok(())
}
