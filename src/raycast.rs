use bevy::prelude::*;
use bevy_mod_raycast::{
    DefaultPluginState, DefaultRaycastingPlugin, RayCastMethod, RayCastSource, RaycastSystem,
};

use crate::armor_stand::Pivot;

pub struct RaycastPlugin;

impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DefaultPluginState::<RaycastSet>::default().with_debug_cursor())
            .add_plugin(DefaultRaycastingPlugin::<RaycastSet>::default())
            .add_system_to_stage(
                CoreStage::PreUpdate,
                update_raycast_with_cursor.before(RaycastSystem::BuildRays),
            );
    }
}

pub struct RaycastSet;

fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut transform_query: Query<(&mut Transform, &Pivot)>,
    mut query: Query<&mut RayCastSource<RaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query.iter_mut() {
        pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);

        for (entity, intersections) in pick_source.intersections_mut() {
            if let Ok((mut transform, pivot)) = transform_query.get_mut(*entity) {
                let rot = Quat::from_rotation_x(0.05);
                let pivot_vec = Vec3::new(pivot.0, pivot.1, pivot.2);

                transform.translation = rot * (transform.translation - pivot_vec) + pivot_vec;
                transform.rotation = rot * transform.rotation;
            }
        }
    }
}
