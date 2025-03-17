use crate::{
    ball::{self},
    island::{self},
    main_menu::{self},
    player::{self},
    resolution,
    score::{self},
};
use bevy::prelude::*;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState { running: false })
            .insert_resource(ClearColor(Color::BLACK))
            .add_event::<GameStartEvent>()
            .add_plugins((
                resolution::ResolutionPlugin,
                main_menu::MenuPlugin,
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

fn update_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut commands: Commands,
    game_state: Res<GameState>,
    mut game_start_events: EventWriter<GameStartEvent>,
) {
    if keys.pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    } else if !game_state.running && keys.just_released(KeyCode::Enter) {
        commands.insert_resource(GameState { running: true });
        game_start_events.send(GameStartEvent);
    }
}
