use crate::{ball, island, player, resolution, score};
use bevy::prelude::*;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState { running: false })
            .add_event::<GameStartEvent>()
            .add_plugins((
                resolution::ResolutionPlugin,
                player::PlayerPlugin,
                island::IslandPlugin,
                ball::BallPlugin,
                score::ScorePlugin,
            ))
            .add_systems(Startup, setup_scene)
            .add_systems(Update, update_game);
    }
}
#[derive(Resource, PartialEq, Eq)]
pub struct GameState {
    pub running: bool,
}

#[derive(Event)]
pub struct GameStartEvent;

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d { ..default() });
}

fn update_game(keys: Res<ButtonInput<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keys.pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    }
}
