use crate::game::GameState;
use crate::resolution;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_player.run_if(resource_equals(GameState { running: true })),),
        );
    }
}

#[derive(Component)]
pub(crate) struct Player {}
pub const PLAYER_WIDTH: f32 = 100.;
pub const PLAYER_HEIGT: f32 = 20.;
pub fn setup_player(mut commands: Commands, resolution: Res<resolution::Resolution>) {
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
    mut mouse_motion: EventReader<MouseMotion>,
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

    for event in mouse_motion.read() {
        transform.translation.x += event.delta.x;
    }

    let left_bound = (-resolution.screen_dimensions.x * 0.5) + (PLAYER_WIDTH / 2.);
    let right_bound = (resolution.screen_dimensions.x * 0.5) - (PLAYER_WIDTH / 2.);

    transform.translation.x = transform.translation.x.clamp(left_bound, right_bound)
}
