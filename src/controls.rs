use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::armor_stand::{ArmorStand, ArmorStandBundle, ArmorStandRequester, Rotator};
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(controllable_system);
    }
}

#[derive(Component, Default)]
pub struct Controllable {
    pub label: String,
}

fn controllable_system(
    mut egui_context: ResMut<EguiContext>,
    mut stand_query: Query<(&ArmorStand, &mut Transform)>,
    mut parts_query: Query<(&mut Rotator, &Controllable)>,
    mut requester: ResMut<ArmorStandRequester>,
) {
    egui::Window::new("Armor Stand Controls")
        .default_width(500.0)
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.add_enabled_ui(true, |ui| {
                if ui.button("New armor stand").clicked() {
                    requester.request_armor_stand();
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

                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        for (mut rotator, controllable) in parts_query.iter_mut() {
                            ui.label(controllable.label.clone());

                            ui.add(
                                egui::Slider::new(&mut rotator.euler.x, -180.0..=180.0)
                                    .show_value(false),
                            );

                            ui.add(
                                egui::Slider::new(&mut rotator.euler.y, -180.0..=180.0)
                                    .show_value(false),
                            );

                            ui.add(
                                egui::Slider::new(&mut rotator.euler.z, -180.0..=180.0)
                                    .show_value(false),
                            );

                            ui.end_row();
                        }
                    });
            });
        });
}
