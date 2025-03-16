use std::{fs, io::Write, path::PathBuf};

use bevy::prelude::*;
use dirs::data_dir;
use serde::{Deserialize, Serialize};

use crate::{
    game::{GameStartEvent, GameState},
    resolution,
};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .insert_resource(load_save_data())
            .add_systems(
                Update,
                (
                    spawn_score,
                    update_score_display.run_if(resource_equals(GameState { running: true })),
                ),
            );
    }
}

const SAVE_FILE: &str = "save_data.json";
#[derive(Resource, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SaveData {
    high_score: i32,
    last_score: i32,
}

#[derive(Resource)]
pub struct Score(pub i32);

fn get_save_path() -> PathBuf {
    let mut path = data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("IslandPong");
    fs::create_dir_all(&path).ok();
    path.push(SAVE_FILE);
    path
}

fn load_save_data() -> SaveData {
    let save_path = get_save_path();
    if let Ok(data) = fs::read_to_string(save_path) {
        if let Ok(save_data) = serde_json::from_str(&data) {
            println!("Saved data: {:?}", save_data);
            return save_data;
        }
    }
    SaveData {
        high_score: -1,
        last_score: -1,
    }
}

pub fn save_data(score: i32, mut current_data: SaveData) -> SaveData {
    current_data.last_score = score;
    if score > current_data.high_score {
        current_data.high_score = score;
    }

    let save_path = get_save_path();
    if let Ok(json) = serde_json::to_string_pretty(&current_data) {
        let _ = fs::File::create(save_path).and_then(|mut file| file.write_all(json.as_bytes()));
        println!("Data saved");
    }

    current_data
}

#[derive(Component)]
pub struct ScoreText;

fn spawn_score(
    mut commands: Commands,
    resolution: Res<resolution::Resolution>,
    mut game_start_events: EventReader<GameStartEvent>,
) {
    for _ in game_start_events.read() {
        commands.insert_resource(Score(0));
        commands.spawn((
            Text::new("0"),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(resolution.screen_dimensions.y / 2.),
                right: Val::Px(resolution.screen_dimensions.x / 2.),
                ..default()
            },
            ScoreText,
        ));
    }
}

fn update_score_display(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        let mut text = match query.get_single_mut() {
            Ok(text) => text,
            Err(_) => return,
        };
        **text = format!("{}", score.0);
    }
}
