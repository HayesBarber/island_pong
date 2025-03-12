use crate::resolution;
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
    }
}

fn setup_player(mut commands: Commands, resolution: Res<resolution::Resolution>) {
    commands.spawn((
        Sprite {
            color: Color::srgb(1., 1., 1.),
            custom_size: Some(Vec2::new(100.0, 20.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            0.,
            (-resolution.screen_dimensions.y / 2.) + 20.,
            0.,
        )),
    ));
}
