use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{island::ISLAND_HEIGHT, resolution};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ball);
    }
}

#[derive(Component)]
struct Ball {}
const BALL_RADIUS: f32 = 15.;
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
        Ball {},
    ));
}
