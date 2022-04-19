use bevy::prelude::*;

use crate::controls::Controllable;

pub struct ArmorStandDummyPlugin;

impl Plugin for ArmorStandDummyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ArmorStandRequester::default())
            .add_startup_system(spawn_armor_stand)
            .add_system(rotator_system)
            .add_system(armor_stand_requester_system);
    }
}

#[derive(Default)]
pub struct ArmorStandRequester {
    request: bool,
}

impl ArmorStandRequester {
    pub fn request(&mut self) {
        self.request = true;
    }

    fn requeted(&mut self) {
        self.request = false;
    }
}

#[derive(Component, Default)]
pub struct ArmorStand;

#[derive(Bundle, Default)]
pub struct ArmorStandBundle {
    armor_stand: ArmorStand,
    transform: Transform,
    global_tranform: GlobalTransform,
}

fn spawn_armor_stand(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn()
        .insert_bundle(ArmorStandBundle::default())
        .with_children(|mut child_builder| {
            create_armor_stand_part(
                &mut child_builder,
                &mut meshes,
                &mut materials,
                "Head".to_owned(),
                Vec3::new(0.0, 22.0, 0.0),
                vec![BoxProps {
                    position: Vec3::new(0.0, 3.5, 0.0),
                    size: Vec3::new(2.0, 7.0, 2.0),
                }],
            );

            create_armor_stand_part(
                &mut child_builder,
                &mut meshes,
                &mut materials,
                "Body".to_owned(),
                Vec3::new(0.0, 23.0, 0.0),
                vec![
                    BoxProps {
                        position: Vec3::new(0.0, -11.0, 0.0),
                        size: Vec3::new(8.0, 2.0, 2.0),
                    },
                    BoxProps {
                        position: Vec3::new(-2.0, -6.5, 0.0),
                        size: Vec3::new(2.0, 7.0, 2.0),
                    },
                    BoxProps {
                        position: Vec3::new(2.0, -6.5, 0.0),
                        size: Vec3::new(2.0, 7.0, 2.0),
                    },
                    BoxProps {
                        position: Vec3::new(0.0, -1.5, 0.0),
                        size: Vec3::new(12.0, 3.0, 3.0),
                    },
                ],
            );

            create_armor_stand_part(
                &mut child_builder,
                &mut meshes,
                &mut materials,
                "Left Arm".to_owned(),
                Vec3::new(6.0, 21.0, 0.0),
                vec![BoxProps {
                    position: Vec3::new(0.0, -4.0, 0.0),
                    size: Vec3::new(2.0, 12.0, 2.0),
                }],
            );

            create_armor_stand_part(
                &mut child_builder,
                &mut meshes,
                &mut materials,
                "Right Arm".to_owned(),
                Vec3::new(-6.0, 21.0, 0.0),
                vec![BoxProps {
                    position: Vec3::new(0.0, -4.0, 0.0),
                    size: Vec3::new(2.0, 12.0, 2.0),
                }],
            );

            create_armor_stand_part(
                &mut child_builder,
                &mut meshes,
                &mut materials,
                "Left Leg".to_owned(),
                Vec3::new(2.0, 11.0, 0.0),
                vec![BoxProps {
                    position: Vec3::new(0.0, -5.5, 0.0),
                    size: Vec3::new(2.0, 11.0, 2.0),
                }],
            );

            create_armor_stand_part(
                &mut child_builder,
                &mut meshes,
                &mut materials,
                "Right Leg".to_owned(),
                Vec3::new(-2.0, 11.0, 0.0),
                vec![BoxProps {
                    position: Vec3::new(0.0, -5.5, 0.0),
                    size: Vec3::new(2.0, 11.0, 2.0),
                }],
            );
        });
}

#[derive(Bundle, Default)]
struct ArmorStandPartBundle {
    transform: Transform,
    visibility: Visibility,
    global_tranform: GlobalTransform,
    computed_visiblit: ComputedVisibility,
    rotator: Rotator,
    controllable: Controllable,
}

struct BoxProps {
    position: Vec3,
    size: Vec3,
}

fn create_armor_stand_part(
    child_builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    label: String,
    position: Vec3,
    boxes: Vec<BoxProps>,
) {
    child_builder
        .spawn_bundle(ArmorStandPartBundle {
            transform: Transform::from_xyz(
                convert_to_units(position.x),
                convert_to_units(position.y),
                convert_to_units(position.z),
            ),
            controllable: Controllable { label },
            ..Default::default()
        })
        .with_children(|parent| {
            for part in boxes.iter() {
                parent.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(
                        convert_to_units(part.size.x),
                        convert_to_units(part.size.y),
                        convert_to_units(part.size.z),
                    ))),
                    material: materials.add(Color::rgba(0.8, 0.7, 0.6, 0.5).into()),
                    transform: Transform::from_xyz(
                        convert_to_units(part.position.x),
                        convert_to_units(part.position.y),
                        convert_to_units(part.position.z),
                    ),
                    ..Default::default()
                });
            }
        });
}

fn convert_to_units(value: f32) -> f32 {
    value as f32 / 16.0
}

#[derive(Component, Default, Debug)]
pub struct Rotator {
    pub euler: Vec3,
}

fn armor_stand_requester_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut requester: ResMut<ArmorStandRequester>,
) {
    if requester.request {
        spawn_armor_stand(commands, meshes, materials);
        requester.requeted();
    }
}

pub fn rotator_system(mut query: Query<(&mut Transform, &Rotator)>) {
    for (mut transform, rotator) in query.iter_mut() {
        *transform = Transform::from_rotation(
            Quat::from_rotation_x(rotator.euler.x.to_radians())
                * Quat::from_rotation_y(rotator.euler.y.to_radians())
                * Quat::from_rotation_z(rotator.euler.z.to_radians()),
        )
        .with_translation(transform.translation);
    }
}
