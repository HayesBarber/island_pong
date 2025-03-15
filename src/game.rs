use crate::{
    ball::{self, spawn_ball},
    island::{self, spawn_island},
    player::{self, spawn_player},
    resolution,
    score::{self, spawn_score},
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
    mut commands: Commands,
    resolution: Res<resolution::Resolution>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<GameState>,
) {
    if keys.pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    } else if !game_state.running && keys.just_released(KeyCode::Enter) {
        commands.insert_resource(GameState { running: true });
        start_game(&mut commands, resolution, meshes, materials);
    }
}

fn start_game(
    commands: &mut Commands,
    resolution: Res<resolution::Resolution>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_island(commands, &resolution);
    spawn_player(commands, &resolution);
    spawn_score(commands, &resolution);
    spawn_ball(commands, resolution, meshes, materials);
}
