use bevy::prelude::*;

pub mod game;
pub mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Island Pong"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(400., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            game::GamePlugin,
        ))
        .run();
}
