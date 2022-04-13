use bevy::prelude::*;
use bevy_mod_picking::*;

mod camera;
use camera::{pan_orbit_camera, PanOrbitCamera};

mod armor_stand;
use armor_stand::ArmorStandDummyPlugin;

mod controls;
use controls::ControlsPlugin;

fn main() {
    App::new()
        .insert_resource(PhysicsTimer(Timer::from_seconds(1.0 / 60.0, true)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ControlsPlugin)
        .add_plugin(ArmorStandDummyPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(setup)
        .add_system(pan_orbit_camera)
        .add_system(physics_timer_tick)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        .insert_bundle(PickingCameraBundle::default())
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        });
}
pub struct PhysicsTimer(Timer);

fn physics_timer_tick(time: Res<Time>, mut timer: ResMut<PhysicsTimer>) {
    timer.0.tick(time.delta());
}
