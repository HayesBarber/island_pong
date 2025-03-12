use crate::resolution;
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, update_player);
    }
}

#[derive(Component)]
struct Player {}
const PLAYER_WIDTH: f32 = 100.;
const PLAYER_HEIGT: f32 = 20.;
fn setup_player(mut commands: Commands, resolution: Res<resolution::Resolution>) {
    commands.spawn((
        Sprite {
            color: Color::srgb(1., 1., 1.),
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGT)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            0.,
            (-resolution.screen_dimensions.y / 2.) + PLAYER_HEIGT,
            0.,
        )),
        Player {},
    ));
}

const SPEED: f32 = 400.;
fn update_player(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<resolution::Resolution>,
) {
    let (mut _player, mut transform) = player_query.single_mut();

    let mut horizontal = 0.;

    if keys.pressed(KeyCode::KeyA)
        || keys.pressed(KeyCode::KeyJ)
        || keys.pressed(KeyCode::ArrowLeft)
    {
        horizontal += -1.;
    }
    if keys.pressed(KeyCode::KeyD)
        || keys.pressed(KeyCode::KeyK)
        || keys.pressed(KeyCode::ArrowRight)
    {
        horizontal += 1.;
    }
    transform.translation.x += horizontal * time.delta_secs() * SPEED;

    let left_bound = (-resolution.screen_dimensions.x * 0.5) + (PLAYER_WIDTH / 2.);
    let right_bound = (resolution.screen_dimensions.x * 0.5) - (PLAYER_WIDTH / 2.);

    if transform.translation.x > right_bound {
        transform.translation.x = right_bound;
    }
    if transform.translation.x < left_bound {
        transform.translation.x = left_bound;
    }
}
