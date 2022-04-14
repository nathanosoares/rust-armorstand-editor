use bevy::prelude::*;

mod camera;
use bevy_mod_raycast::RayCastSource;
use camera::{pan_orbit_camera, PanOrbitCamera};

mod armor_stand;
use armor_stand::ArmorStandDummyPlugin;

mod controls;
use controls::ControlsPlugin;

mod raycast;
use raycast::{RaycastPlugin, RaycastSet};

fn main() {
    App::new()
        .insert_resource(PhysicsTimer(Timer::from_seconds(1.0 / 60.0, true)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ControlsPlugin)
        .add_plugin(ArmorStandDummyPlugin)
        .add_plugin(RaycastPlugin)
        .add_startup_system(setup)
        .add_system(pan_orbit_camera)
        .add_system(physics_timer_tick)
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
    // camera
    let translation = Vec3::new(-1.5, 3.5, 4.0);
    let radius = translation.length();

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        })
        .insert(RayCastSource::<RaycastSet>::new_transform_empty());
}
pub struct PhysicsTimer(Timer);

fn physics_timer_tick(time: Res<Time>, mut timer: ResMut<PhysicsTimer>) {
    timer.0.tick(time.delta());
}
