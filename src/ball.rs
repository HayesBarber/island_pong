use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{island::ISLAND_HEIGHT, resolution};

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
const BALL_SPEED: f32 = 300.0;

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
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (mut transform, mut velocity) in &mut query {
        let delta = time.delta_secs();

        transform.translation.x += velocity.0.x * delta;
        transform.translation.y += velocity.0.y * delta;

        let half_width = resolution.screen_dimensions.x / 2.0;
        let half_height = resolution.screen_dimensions.y / 2.0;
        let ball_radius = BALL_RADIUS;

        if transform.translation.x + ball_radius >= half_width
            || transform.translation.x - ball_radius <= -half_width
        {
            velocity.0.x = -velocity.0.x;
        }
        if transform.translation.y + ball_radius >= half_height {
            velocity.0.y = -velocity.0.y;
        }
        if transform.translation.y - ball_radius <= -half_height {
            app_exit_events.send(AppExit::Success);
        }
    }
}
