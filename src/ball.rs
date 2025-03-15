use bevy::{color::palettes::css::WHITE, prelude::*};
use rand::Rng;

use crate::{
    game::GameState,
    island::{self, ISLAND_HEIGHT, ISLAND_WIDTH},
    player::{self, PLAYER_HEIGT, PLAYER_WIDTH},
    resolution,
    score::{self, ScoreText},
};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_ball.run_if(resource_equals(GameState { running: true })),),
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

fn move_ball(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    resolution: Res<resolution::Resolution>,
    mut score: ResMut<score::Score>,
    mut query_set: ParamSet<(
        Query<(Entity, &mut Transform, &mut Velocity), With<Ball>>,
        Query<(Entity, &Transform), With<player::Player>>,
        Query<(Entity, &Transform, &mut Sprite), With<island::Island>>,
        Query<Entity, With<ScoreText>>,
    )>,
) {
    let player_half_width = PLAYER_WIDTH / 2.;
    let player_half_height = PLAYER_HEIGT / 2.;
    let (player_top, player_x, player_y) = {
        let player_query = query_set.p1();
        let player_transform = player_query.single().1;
        let player_top = player_transform.translation.y + player_half_height;
        let player_x = player_transform.translation.x;
        let player_y = player_transform.translation.y;
        (player_top, player_x, player_y)
    };

    let island_half_width = ISLAND_WIDTH / 2.;
    let (island_entity, island_top, island_x, island_bottom) = {
        let island_query = query_set.p2();
        let island_transform = island_query.single().1;

        let island_top = island_transform.translation.y + ISLAND_HEIGHT / 2.0;
        let island_x = island_transform.translation.x;
        let island_bottom = island_transform.translation.y - ISLAND_HEIGHT / 2.0;

        let island_entity = island_query.single().0;

        (island_entity, island_top, island_x, island_bottom)
    };

    let half_width = resolution.screen_dimensions.x / 2.0;
    let half_height = resolution.screen_dimensions.y / 2.0;
    let ball_radius = BALL_RADIUS;
    let delta = time.delta_secs();

    let count = query_set.p0().iter().count();
    let mut spawn_velocity: Option<Vec2> = None;

    for (entity, mut ball_transform, mut velocity) in query_set.p0().iter_mut() {
        ball_transform.translation.x += velocity.0.x * delta;
        ball_transform.translation.y += velocity.0.y * delta;
        let ball_bottom = ball_transform.translation.y - ball_radius;
        let ball_top = ball_transform.translation.y + ball_radius;

        // ball hits left or right walls
        if ball_transform.translation.x + ball_radius >= half_width {
            velocity.0.x = -velocity.0.x.abs();
        } else if ball_transform.translation.x - ball_radius <= -half_width {
            velocity.0.x = velocity.0.x.abs();
        }
        // ball hits ceiling
        if ball_top >= half_height {
            velocity.0.y = -velocity.0.y.abs();
        }
        //ball made it past paddle
        if ball_bottom <= -half_height {
            commands.entity(entity).despawn();
            if count <= 1 {
                commands.insert_resource(GameState { running: false });
                commands.entity(island_entity).despawn();
            }
        }
        //ball hits paddle
        let x_collision = ball_transform.translation.x >= player_x - player_half_width
            && ball_transform.translation.x <= player_x + player_half_width;
        let y_collision = ball_bottom <= player_top && ball_bottom >= player_y;
        if x_collision && y_collision {
            velocity.0.y = velocity.0.y.abs();
        }
        //ball hits island
        let island_x_collision = ball_transform.translation.x >= island_x - island_half_width
            && ball_transform.translation.x <= island_x + island_half_width;
        let island_y_collision = ball_top >= island_bottom && ball_bottom <= island_top;

        if island_x_collision && island_y_collision {
            velocity.0.y = -velocity.0.y.abs();
            score.0 += 1;

            if score.0 > 0 && score.0 % 5 == 0 {
                velocity.0 *= 1.2;
                spawn_velocity = Some(velocity.0);
            }
        }
    }
    if spawn_velocity.is_some() {
        spawn_ball_with_velocity(
            &mut commands,
            resolution,
            meshes,
            materials,
            get_random_velocity(spawn_velocity.unwrap().length()),
        );
        if let Some(size) = &mut query_set.p2().single_mut().2.custom_size {
            size.x = (size.x - 5.).max(75.);
        }
    }
}
