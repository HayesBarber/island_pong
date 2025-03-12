use bevy::prelude::*;

use crate::resolution;

pub struct IslandPlugin;
impl Plugin for IslandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_island);
    }
}

#[derive(Component)]
struct Island;
const ISLAND_WIDTH: f32 = 200.;
const ISLAND_HEIGHT: f32 = 20.;

fn setup_island(mut commands: Commands, resolution: Res<resolution::Resolution>) {
    commands.spawn((
        Sprite {
            color: Color::srgb(1., 1., 1.),
            custom_size: Some(Vec2::new(ISLAND_WIDTH, ISLAND_HEIGHT)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            0.,
            (resolution.screen_dimensions.y / 2.) - ISLAND_HEIGHT,
            0.,
        )),
        Island {},
    ));
}
