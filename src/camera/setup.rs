use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 10., 4.).looking_at(Vec3::ZERO, Vec3::Y),
        MeshPickingCamera,
    ));
}

pub fn setup_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1., 0.8, 1.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
