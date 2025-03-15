use crate::{
    ball::{self, spawn_ball},
    island, player, resolution, score,
};
use bevy::prelude::*;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState { running: false })
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

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d { ..default() });
}

fn update_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<GameState>,
) {
    if keys.pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    } else if keys.just_released(KeyCode::Enter) {
        game_state.running = true;
    }
}

fn start_game(
    mut commands: Commands,
    resolution: Res<resolution::Resolution>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_ball(commands, resolution, meshes, materials);
}
