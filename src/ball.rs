use bevy::{color::palettes::css::WHITE, prelude::*};
use rand::Rng;

use crate::{
    island::{self, ISLAND_HEIGHT, ISLAND_WIDTH},
    player::{self, PLAYER_HEIGT, PLAYER_WIDTH},
    resolution, score,
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
const BALL_SPEED: f32 = 450.;

fn setup_ball(
    mut commands: Commands,
    resolution: Res<resolution::Resolution>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    let random_x = rng.random_range(-1.0..1.0);
    let initial_velocity = Vec2::new(random_x, -1.0).normalize() * BALL_SPEED;

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
        Transform::from_translation(Vec3::new(
            0.,
            (resolution.screen_dimensions.y / 2.) - (ISLAND_HEIGHT * 3.),
            0.,
        )),
        Ball,
        Velocity(initial_velocity),
    ));
}

fn move_ball(
    time: Res<Time>,
    resolution: Res<resolution::Resolution>,
    mut score: ResMut<score::Score>,
    mut query_set: ParamSet<(
        Query<(&mut Transform, &mut Velocity), With<Ball>>,
        Query<&Transform, With<player::Player>>,
        Query<&Transform, With<island::Island>>,
    )>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    let player_half_width = PLAYER_WIDTH / 2.;
    let player_half_height = PLAYER_HEIGT / 2.;
    let (player_top, player_x, player_y) = {
        let player_query = query_set.p1();
        let player_transform = player_query.single();
        let player_top = player_transform.translation.y + player_half_height;
        let player_x = player_transform.translation.x;
        let player_y = player_transform.translation.y;
        (player_top, player_x, player_y)
    };

    let island_half_width = ISLAND_WIDTH / 2.;
    let (island_top, island_x, island_bottom) = {
        let island_query = query_set.p2();
        let island_transform = island_query.single();

        let island_top = island_transform.translation.y + ISLAND_HEIGHT / 2.0;
        let island_x = island_transform.translation.x;
        let island_bottom = island_transform.translation.y - ISLAND_HEIGHT / 2.0;

        (island_top, island_x, island_bottom)
    };

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
        let ball_bottom = ball_transform.translation.y - ball_radius;
        let x_collision = ball_transform.translation.x >= player_x - player_half_width
            && ball_transform.translation.x <= player_x + player_half_width;
        let y_collision = ball_bottom <= player_top && ball_bottom >= player_y;

        if x_collision && y_collision {
            velocity.0.y = velocity.0.y.abs();
        }

        let island_x_collision = ball_transform.translation.x >= island_x - island_half_width
            && ball_transform.translation.x <= island_x + island_half_width;
        let island_y_collision = ball_transform.translation.y + ball_radius >= island_bottom
            && ball_transform.translation.y - ball_radius <= island_top;

        let hitting_left_side =
            ball_transform.translation.x - ball_radius <= island_x - island_half_width;
        let hitting_right_side =
            ball_transform.translation.x + ball_radius >= island_x + island_half_width;

        if island_x_collision && island_y_collision {
            if hitting_left_side || hitting_right_side {
                velocity.0.x = -velocity.0.x;
            }
            velocity.0.y = -velocity.0.y.abs();
            score.0 += 1;

            if score.0 % 5 == 0 {
                velocity.0 *= 1.2;
            }
        }
    }
}
