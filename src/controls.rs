use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::armor_stand::Rotator;

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
    mut query: Query<(&mut Rotator, &Controllable)>,
) {
    egui::Window::new("Armor Stand Controls")
        .default_width(500.0)
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.add_enabled_ui(true, |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        for (mut rotator, controllable) in query.iter_mut() {
                            ui.label(controllable.label.clone());

                            ui.add(
                                egui::Slider::new(&mut rotator.euler.x, 0.0..=360.0)
                                    .show_value(false),
                            );

                            ui.add(
                                egui::Slider::new(&mut rotator.euler.y, 0.0..=360.0)
                                    .show_value(false),
                            );

                            ui.add(
                                egui::Slider::new(&mut rotator.euler.z, 0.0..=360.0)
                                    .show_value(false),
                            );

                            ui.end_row();
                        }
                    });
            });
        });
}
