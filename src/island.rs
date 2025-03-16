use bevy::prelude::*;

use crate::{game::GameStartEvent, resolution};

pub struct IslandPlugin;
impl Plugin for IslandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_island,));
    }
}

#[derive(Component)]
pub(crate) struct Island;
pub const ISLAND_WIDTH: f32 = 200.;
pub const ISLAND_HEIGHT: f32 = 20.;

fn spawn_island(
    mut commands: Commands,
    resolution: Res<resolution::Resolution>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_start_events: EventReader<GameStartEvent>,
) {
    for _ in game_start_events.read() {
        commands.spawn((
            Mesh2d(meshes.add(Capsule2d::new(
                ISLAND_HEIGHT / 2.,
                ISLAND_WIDTH - ISLAND_HEIGHT / 2.,
            ))),
            MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
            Transform {
                translation: Vec3::new(
                    0.,
                    (resolution.screen_dimensions.y / 2.) - (ISLAND_HEIGHT / 2.) - 10.,
                    0.,
                ),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                ..default()
            },
            Island {},
        ));
    }
}
