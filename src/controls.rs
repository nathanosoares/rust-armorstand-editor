use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::{
    armor_stand::{ArmorStand, ArmorStandRequester, Rotator},
    camera::{PanOrbitCameraTransform, CAMERA_TARGET},
};
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .init_resource::<OccupiedScreenSpace>()
            .add_system(controllable_system);
    }
}

#[derive(Component, Default)]
pub struct Controllable {
    pub label: String,
}

#[derive(Default)]
pub struct OccupiedScreenSpace {
    pub right: f32,
    pub last_right: f32,
}

fn controllable_system(
    mut egui_context: ResMut<EguiContext>,
    mut stand_query: Query<(&ArmorStand, &mut Transform)>,
    mut parts_query: Query<(&mut Rotator, &Controllable)>,
    mut requester: ResMut<ArmorStandRequester>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    occupied_screen_space.last_right = occupied_screen_space.right;
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.add_enabled_ui(true, |ui| {
                if ui.button("New armor stand").clicked() {
                    requester.request();
                }

                for (_, mut transform) in stand_query.iter_mut() {
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut transform.translation.x)
                                .speed(0.1)
                                .fixed_decimals(2),
                        )
                        .on_hover_text("Move X");

                        ui.add(
                            egui::DragValue::new(&mut transform.translation.y)
                                .speed(0.1)
                                .fixed_decimals(2),
                        )
                        .on_hover_text("Move Y");
                        ui.add(
                            egui::DragValue::new(&mut transform.translation.z)
                                .speed(0.1)
                                .fixed_decimals(2),
                        )
                        .on_hover_text("Move Z");
                    });
                }

                ui.separator();

                for (mut rotator, controllable) in parts_query.iter_mut() {
                    ui.add(egui::Label::new(controllable.label.clone()));

                    ui.end_row();

                    egui::Grid::new(format!("grid_{}", controllable.label.clone()))
                        .num_columns(3)
                        .spacing([10.0, 4.0])
                        .show(ui, |ui| {
                            ui.add(
                                egui::DragValue::new(&mut rotator.euler.x)
                                    .speed(1.0)
                                    .clamp_range(-180.0..=180.0)
                                    .fixed_decimals(0),
                            )
                            .on_hover_text(format!("Rotate {} X", controllable.label.clone()));

                            ui.add(
                                egui::DragValue::new(&mut rotator.euler.y)
                                    .speed(1.0)
                                    .clamp_range(-180.0..=180.0)
                                    .fixed_decimals(0),
                            )
                            .on_hover_text(format!("Rotate {} Y", controllable.label.clone()));

                            ui.add(
                                egui::DragValue::new(&mut rotator.euler.z)
                                    .speed(1.0)
                                    .clamp_range(-180.0..=180.0)
                                    .fixed_decimals(0),
                            )
                            .on_hover_text(format!("Rotate {} Z", controllable.label.clone()));
                        });
                }
            });
        })
        .response
        .rect
        .width();
}
