use bevy::prelude::*;

use crate::resolution;

pub struct IslandPlugin;
impl Plugin for IslandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_island);
    }
}

#[derive(Component)]
pub(crate) struct Island;
pub const ISLAND_WIDTH: f32 = 200.;
pub const ISLAND_HEIGHT: f32 = 20.;

pub fn setup_island(mut commands: Commands, resolution: Res<resolution::Resolution>) {
    commands.spawn((
        Sprite {
            color: Color::srgb(1., 1., 1.),
            custom_size: Some(Vec2::new(ISLAND_WIDTH, ISLAND_HEIGHT)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            0.,
            (resolution.screen_dimensions.y / 2.) - (ISLAND_HEIGHT / 2.) - 10.,
            0.,
        )),
        Island {},
    ));
}
