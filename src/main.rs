use bevy::prelude::*;

mod camera;
use camera::{pan_orbit_camera, CameraPlugin};

mod armor_stand;
use armor_stand::ArmorStandDummyPlugin;

mod controls;
use controls::ControlsPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(ControlsPlugin)
        .add_plugin(ArmorStandDummyPlugin)
        .add_startup_system(setup)
        .add_system(pan_orbit_camera)
        .run();
}

fn setup(mut commands: Commands) {
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
