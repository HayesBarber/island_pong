use bevy::{color::palettes::css::WHITE, prelude::*};
use rand::Rng;

use crate::{
    game::GameState,
    island::{self, ISLAND_HEIGHT, ISLAND_WIDTH},
    player::{self, PLAYER_HEIGT, PLAYER_WIDTH},
    resolution,
    score::{self, ScoreText, save_data},
};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_ball,
                ball_wall_collision,
                ball_paddle_collision,
                ball_island_collision,
                ball_despawn,
            )
                .run_if(resource_equals(GameState { running: true })),
        );
    }
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity(Vec2);

pub const BALL_RADIUS: f32 = 12.;
pub const BALL_SPEED: f32 = 450.;

fn get_random_velocity(ball_speed: f32) -> Vec2 {
    let mut rng = rand::rng();
    let random_x = rng.random_range(-1.0..1.0);
    return Vec2::new(random_x, -1.0).normalize() * ball_speed;
}

pub fn spawn_ball(
    commands: &mut Commands,
    resolution: Res<resolution::Resolution>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_ball_with_velocity(
        commands,
        resolution,
        meshes,
        materials,
        get_random_velocity(BALL_SPEED),
    );
}

fn spawn_ball_with_velocity(
    commands: &mut Commands,
    resolution: Res<resolution::Resolution>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    velocity: Vec2,
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
        Velocity(velocity),
    ));
}

fn move_ball(mut query: Query<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    let delta = time.delta_secs();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * delta;
        transform.translation.y += velocity.0.y * delta;
    }
}

// Ball Collision with Walls
fn ball_wall_collision(
    mut query: Query<(&mut Velocity, &Transform), With<Ball>>,
    resolution: Res<resolution::Resolution>,
) {
    let half_width = resolution.screen_dimensions.x / 2.0;
    let half_height = resolution.screen_dimensions.y / 2.0;

    for (mut velocity, transform) in query.iter_mut() {
        let ball_x = transform.translation.x;
        let ball_y = transform.translation.y;

        if ball_x + BALL_RADIUS >= half_width {
            velocity.0.x = -velocity.0.x.abs();
        } else if ball_x - BALL_RADIUS <= -half_width {
            velocity.0.x = velocity.0.x.abs();
        }

        if ball_y + BALL_RADIUS >= half_height {
            velocity.0.y = -velocity.0.y.abs();
        }
    }
}

fn ball_paddle_collision(
    mut query: Query<(&mut Velocity, &Transform), With<Ball>>,
    player_query: Query<&Transform, With<player::Player>>,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    let player_half_width = PLAYER_WIDTH / 2.;
    let player_half_height = PLAYER_HEIGT / 2.;
    let player_x = player_transform.translation.x;
    let player_top = player_transform.translation.y + player_half_height;
    let player_bottom = player_transform.translation.y;

    for (mut velocity, transform) in query.iter_mut() {
        let ball_x = transform.translation.x;
        let ball_bottom = transform.translation.y - BALL_RADIUS;

        let x_collision =
            ball_x >= player_x - player_half_width && ball_x <= player_x + player_half_width;
        let y_collision = ball_bottom <= player_top && ball_bottom >= player_bottom;

        if x_collision && y_collision {
            velocity.0.y = velocity.0.y.abs();
        }
    }
}

fn ball_island_collision(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    mut island_query: Query<(&Transform, &mut Sprite), With<island::Island>>,
    mut score: ResMut<score::Score>,
    resolution: Res<resolution::Resolution>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let (island_transform, mut island_sprite) = match island_query.get_single_mut() {
        Ok(island) => island,
        Err(_) => return,
    };

    let island_half_width = ISLAND_WIDTH / 2.;
    let island_x = island_transform.translation.x;
    let island_top = island_transform.translation.y + ISLAND_HEIGHT / 2.0;
    let island_bottom = island_transform.translation.y - ISLAND_HEIGHT / 2.0;

    let mut spawn_velocity: Option<Vec2> = None;

    for (mut velocity, transform) in ball_query.iter_mut() {
        let ball_x = transform.translation.x;
        let ball_top = transform.translation.y + BALL_RADIUS;
        let ball_bottom = transform.translation.y - BALL_RADIUS;

        let x_collision =
            ball_x >= island_x - island_half_width && ball_x <= island_x + island_half_width;
        let y_collision = ball_top >= island_bottom && ball_bottom <= island_top;

        if x_collision && y_collision {
            velocity.0.y = -velocity.0.y.abs();
            score.0 += 1;

            if score.0 % 5 == 0 {
                velocity.0 *= 1.2;
                spawn_velocity = Some(velocity.0);
            }
        }
    }

    if let Some(new_velocity) = spawn_velocity {
        spawn_ball_with_velocity(
            &mut commands,
            resolution,
            meshes,
            materials,
            get_random_velocity(new_velocity.length()),
        );

        if let Some(size) = &mut island_sprite.custom_size {
            size.x = (size.x - 5.).max(75.);
        }
    }
}

fn ball_despawn(
    mut commands: Commands,
    ball_query: Query<(Entity, &Transform), With<Ball>>,
    resolution: Res<resolution::Resolution>,
    player_query: Query<Entity, With<player::Player>>,
    island_query: Query<Entity, With<island::Island>>,
    score_query: Query<Entity, With<ScoreText>>,
    score: Res<score::Score>,
    saved_data: Res<score::SaveData>,
) {
    let half_height = resolution.screen_dimensions.y / 2.0;

    for (entity, transform) in ball_query.iter() {
        if transform.translation.y - BALL_RADIUS <= -half_height {
            commands.entity(entity).despawn();

            if ball_query.iter().count() <= 1 {
                commands.insert_resource(GameState { running: false });
                if let Ok(player_entity) = player_query.get_single() {
                    commands.entity(player_entity).despawn();
                }
                if let Ok(island_entity) = island_query.get_single() {
                    commands.entity(island_entity).despawn();
                }
                if let Ok(score_entity) = score_query.get_single() {
                    commands.entity(score_entity).despawn();
                }
                commands.insert_resource(save_data(score.0, *saved_data));
            }
        }
    }
}
