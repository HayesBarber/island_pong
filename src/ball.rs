use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    island::ISLAND_HEIGHT,
    player::{self, PLAYER_HEIGT, PLAYER_WIDTH},
    resolution,
};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ball)
            .add_systems(Update, move_ball);
    }
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity(Vec2);

const BALL_RADIUS: f32 = 12.;
const BALL_SPEED: f32 = 400.;

fn setup_ball(
    mut commands: Commands,
    resolution: Res<resolution::Resolution>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
        Transform::from_translation(Vec3::new(
            0.,
            (resolution.screen_dimensions.y / 2.) - (ISLAND_HEIGHT * 3.),
            0.,
        )),
        Ball,
        Velocity(Vec2::new(1.0, -1.0).normalize() * BALL_SPEED),
    ));
}
fn move_ball(
    time: Res<Time>,
    resolution: Res<resolution::Resolution>,
    mut query_set: ParamSet<(
        Query<(&mut Transform, &mut Velocity), With<Ball>>,
        Query<&Transform, With<player::Player>>,
    )>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    let player_transform = query_set.p1().single().clone();

    for (mut ball_transform, mut velocity) in query_set.p0().iter_mut() {
        let delta = time.delta_secs();

        ball_transform.translation.x += velocity.0.x * delta;
        ball_transform.translation.y += velocity.0.y * delta;

        let half_width = resolution.screen_dimensions.x / 2.0;
        let half_height = resolution.screen_dimensions.y / 2.0;
        let ball_radius = BALL_RADIUS;

        if ball_transform.translation.x + ball_radius >= half_width
            || ball_transform.translation.x - ball_radius <= -half_width
        {
            velocity.0.x = -velocity.0.x;
        }
        if ball_transform.translation.y + ball_radius >= half_height {
            velocity.0.y = -velocity.0.y;
        }
        if ball_transform.translation.y - ball_radius <= -half_height {
            app_exit_events.send(AppExit::Success);
        }

        let player_half_width = PLAYER_WIDTH / 2.0;
        let player_half_height = PLAYER_HEIGT / 2.0;

        let ball_bottom = ball_transform.translation.y - ball_radius;
        let player_top = player_transform.translation.y + player_half_height;

        let ball_x = ball_transform.translation.x;
        let player_x = player_transform.translation.x;

        let x_collision =
            ball_x >= player_x - player_half_width && ball_x <= player_x + player_half_width;
        let y_collision =
            ball_bottom <= player_top && ball_bottom >= player_transform.translation.y;

        if x_collision && y_collision {
            velocity.0.y = velocity.0.y.abs();
        }
    }
}
