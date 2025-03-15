use std::fs;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{game::GameState, resolution};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(load_score()).add_systems(
            Update,
            (update_score_display.run_if(resource_equals(GameState { running: true })),),
        );
    }
}

const SAVE_FILE: &str = "score.json";

#[derive(Resource, Serialize, Deserialize)]
pub struct Score(pub i32);

fn load_score() -> Score {
    if let Ok(data) = fs::read_to_string(SAVE_FILE) {
        if let Ok(save_data) = serde_json::from_str(&data) {
            return save_data;
        }
    }
    Score(0)
}

#[derive(Component)]
pub struct ScoreText;

pub fn spawn_score(commands: &mut Commands, resolution: &resolution::Resolution) {
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

fn update_score_display(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        let mut text = match query.get_single_mut() {
            Ok(text) => text,
            Err(_) => return,
        };
        **text = format!("{}", score.0);
    }
}
