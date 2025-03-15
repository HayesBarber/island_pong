use bevy::prelude::*;

use crate::{
    game::{GameStartEvent, GameState},
    resolution,
};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0)) // Start with score 0
            .add_systems(
                Update,
                (
                    setup_score,
                    update_score_display.run_if(resource_equals(GameState { running: true })),
                )
                    .chain(),
            );
    }
}

#[derive(Resource)]
pub struct Score(pub i32);

#[derive(Component)]
struct ScoreText;

fn setup_score(
    mut commands: Commands,
    resolution: Res<resolution::Resolution>,
    mut start_events: EventReader<GameStartEvent>,
) {
    if start_events.read().count() <= 0 {
        return;
    }
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
        let mut text = query.single_mut();
        **text = format!("{}", score.0);
    }
}
