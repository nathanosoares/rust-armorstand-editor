use std::f32::consts::PI;

use bevy::prelude::*;

use crate::controls::Controllable;

pub struct ArmorStandDummyPlugin;

impl Plugin for ArmorStandDummyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_armor_stand)
            .add_system(rotator_system);
    }
}

#[derive(Bundle, Default)]
struct PartBundle {
    transform: Transform,
    visibility: Visibility,
    global_tranform: GlobalTransform,
    computed_visiblit: ComputedVisibility,
    rotator: Rotator,
    controllable: Controllable,
    // #[bundle]
    // pickable: PickableBundle,
}

fn setup_armor_stand(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Head
    commands
        .spawn_bundle(PartBundle {
            transform: Transform::from_xyz(0.0, convert_to_units(22.0), 0.0),
            controllable: Controllable {
                label: "Head".to_owned(),
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(create_armor_stand_part(
                &mut meshes,
                &mut materials,
                2.0,
                7.0,
                2.0,
                0.0,
                3.5,
                0.0,
            ));
        });

    // Body
    commands
        .spawn_bundle(PartBundle {
            transform: Transform::from_xyz(0.0, convert_to_units(23.0), 0.0),
            controllable: Controllable {
                label: "Body".to_owned(),
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Horizontal Bottom
            parent.spawn_bundle(create_armor_stand_part(
                &mut meshes,
                &mut materials,
                8.0,
                2.0,
                2.0,
                0.0,
                -11.0,
                0.0,
            ));

            // Vertical Right
            parent.spawn_bundle(create_armor_stand_part(
                &mut meshes,
                &mut materials,
                2.0,
                7.0,
                2.0,
                -2.0,
                -6.5,
                0.0,
            ));

            // Vertical Left
            parent.spawn_bundle(create_armor_stand_part(
                &mut meshes,
                &mut materials,
                2.0,
                7.0,
                2.0,
                2.0,
                -6.5,
                0.0,
            ));

            // Horizontal Top
            parent.spawn_bundle(create_armor_stand_part(
                &mut meshes,
                &mut materials,
                12.0,
                3.0,
                3.0,
                0.0,
                -1.5,
                0.0,
            ));
        });

    // Left Arm
    commands
        .spawn_bundle(PartBundle {
            transform: Transform::from_xyz(convert_to_units(6.0), convert_to_units(21.0), 0.0),
            controllable: Controllable {
                label: "Left Arm".to_owned(),
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(create_armor_stand_part(
                &mut meshes,
                &mut materials,
                2.0,
                12.0,
                2.0,
                0.0,
                -4.0,
                0.0,
            ));
        });

    // Right Arm
    commands
        .spawn_bundle(PartBundle {
            transform: Transform::from_xyz(convert_to_units(-6.0), convert_to_units(21.0), 0.0),
            controllable: Controllable {
                label: "Right Arm".to_owned(),
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(create_armor_stand_part(
                &mut meshes,
                &mut materials,
                2.0,
                12.0,
                2.0,
                0.0,
                -4.0,
                0.0,
            ));
        });

    // Left Leg
    commands
        .spawn_bundle(PartBundle {
            transform: Transform::from_xyz(convert_to_units(2.0), convert_to_units(11.0), 0.0),
            controllable: Controllable {
                label: "Left Leg".to_owned(),
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(create_armor_stand_part(
                &mut meshes,
                &mut materials,
                2.0,
                11.0,
                2.0,
                0.0,
                -5.5,
                0.0,
            ));
        });

    // Right Leg
    commands
        .spawn_bundle(PartBundle {
            transform: Transform::from_xyz(convert_to_units(-2.0), convert_to_units(11.0), 0.0),
            controllable: Controllable {
                label: "Right Leg".to_owned(),
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(create_armor_stand_part(
                &mut meshes,
                &mut materials,
                2.0,
                11.0,
                2.0,
                0.0,
                -5.5,
                0.0,
            ));
        });
}

fn create_armor_stand_part(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    x_length: f32,
    y_length: f32,
    z_length: f32,
    x_pos: f32,
    y_pos: f32,
    z_pos: f32,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            max_x: convert_to_units(x_length / 2.0),
            min_x: -convert_to_units(x_length / 2.0),
            max_y: convert_to_units(y_length / 2.0),
            min_y: -convert_to_units(y_length / 2.0),
            max_z: convert_to_units(z_length / 2.0),
            min_z: -convert_to_units(z_length / 2.0),
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(
            convert_to_units(x_pos),
            convert_to_units(y_pos),
            convert_to_units(z_pos),
        ),
        ..Default::default()
    }
}

fn convert_to_units(size: f32) -> f32 {
    size as f32 / 16.0
}

#[derive(Component, Default, Debug)]
pub struct Rotator {
    pub euler: Euler,
}

#[derive(Default, Debug)]
pub struct Euler {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn rotator_system(
    // time: Res<Time>,
    // timer: Res<PhysicsTimer>,
    mut query: Query<(&mut Transform, &Rotator)>,
) {
    // if timer.0.just_finished() {
    for (mut transform, rotator) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_x(rotator.euler.x * PI / 180.0)
            * Quat::from_rotation_y(rotator.euler.y * PI / 180.0)
            * Quat::from_rotation_z(rotator.euler.z * PI / 180.0);
        // transform.rotate(Quat::from_rotation_x(0.05));
    }
    // }
}
