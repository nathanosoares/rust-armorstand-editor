use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::armor_stand::Rotator;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(ui_example)
            .add_system(controllable_system);
    }
}

#[derive(Component)]
pub struct Controllable;

pub fn controllable_system(
    mut egui_context: ResMut<EguiContext>,
    mut query: Query<(&Rotator, Added<Controllable>)>,
) {
    for (rotator, added) in query.iter() {
        if added {
            println!("Added: {:?}", rotator)
        }
    }
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Armor Stand Controls")
        .resizable(true)
        .default_width(500.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.add_enabled_ui(true, |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("Left Arm");

                        // ui.add(
                        //     egui::Slider::new(&mut stand_state.left_arm.x, 0.0..=10.0)
                        //         .show_value(false),
                        // );

                        // ui.add(
                        //     egui::Slider::new(&mut stand_state.left_arm.y, 0.0..=10.0)
                        //         .show_value(false),
                        // );

                        // ui.add(
                        //     egui::Slider::new(&mut stand_state.left_arm.z, 0.0..=10.0)
                        //         .show_value(false),
                        // );

                        ui.end_row();
                    });
            });
        });
}
