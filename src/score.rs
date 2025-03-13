use bevy::prelude::*;

use crate::resolution;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0)) // Start with score 0
            .add_systems(Startup, setup_score);
    }
}

#[derive(Resource)]
struct Score(i32);

#[derive(Component)]
struct ScoreText;

fn setup_score(mut commands: Commands, resolution: Res<resolution::Resolution>) {
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
