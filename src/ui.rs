use bevy::{
    math::{Vec2, Vec3},
    prelude::{Res, ResMut, State},
    window::Windows,
};
use bevy_egui::{
    egui::{self, plot::Text, Align2, Checkbox, DragValue, Layout, Pos2},
    EguiContext,
};
use ca::{Dimension, Rule};

use crate::{
    setup::{DEFAULT_X, DEFAULT_Y, DEFAULT_Z},
    utils::{
        simconfig::{self, SimConfig},
        RuleStorage,
    },
};

pub const SIDE_PANEL_WIDTH: f32 = 200.0;

#[derive(Clone, Copy)]
pub struct UiState {
    pub dim: Dimension,
    pub start: bool,
    pub step: bool,
    pub pause: bool,
    pub reset: bool,
    pub noise: f32,
    pub use_noise: bool,
    pub size: Vec3,
    pub zoom_level: f32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            dim: Dimension::D2,
            start: false,
            step: false,
            pause: false,
            reset: false,
            noise: 0.0,
            use_noise: true,
            size: Vec3::new(DEFAULT_X as f32, DEFAULT_Y as f32, 0.),
            zoom_level: 1.,
        }
    }
}

pub fn draw_ui(
    mut egui_context: ResMut<EguiContext>,
    mut rules: ResMut<RuleStorage>,
    mut ui_state: ResMut<UiState>,
    mut sim_config: ResMut<SimConfig>,
    _windows: Res<Windows>,
) {
    // Draw config sidepanel
    egui::SidePanel::left("side_panel")
        .default_width(SIDE_PANEL_WIDTH)
        .resizable(false)
        .max_width(SIDE_PANEL_WIDTH)
        .min_width(SIDE_PANEL_WIDTH)
        .show(egui_context.ctx_mut(), |ui| {
            ui.scope(|ui| {
                ui.allocate_space(egui::Vec2::new(200.0, 10.0));
                ui.heading("Configuration");
                ui.allocate_space(egui::Vec2::new(200.0, 50.0));

                egui::Grid::new("config_grid")
                    .spacing([4.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        draw_config(ui, &mut rules, &mut ui_state, &mut sim_config);
                    });
            });
        });

    // Draw simulation controls
    egui::Window::new("Simulation")
        .anchor(Align2::CENTER_TOP, [-200.0, 5.0])
        .collapsible(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui_state.start = ui.button("Start").clicked();
                ui_state.step = ui.button("Step").clicked();
                ui_state.pause = ui.button("Pause").clicked();
                ui_state.reset = ui.button("Reset").clicked();
            });
        });

    // Draw debug infos
    // TODO find a better solution to drawing struct fields
    egui::Area::new("debug_info")
        .anchor(Align2::RIGHT_TOP, [-20.0, 20.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(format!("dimension: {:?}", ui_state.dim));
        });
}

fn draw_config(
    ui: &mut egui::Ui,
    rules: &mut RuleStorage,
    ui_state: &mut UiState,
    sim_config: &mut SimConfig,
) {
    // Dimensions
    ui.label("Dimension");
    ui.horizontal(|ui| {
        ui.set_enabled(!sim_config.running);
        ui.selectable_value(&mut ui_state.dim, Dimension::D1, "1D");
        ui.selectable_value(&mut ui_state.dim, Dimension::D2, "2D");
        ui.selectable_value(&mut ui_state.dim, Dimension::D3, "3D");
    });
    ui.end_row();

    // Universe size
    ui.label("Size");
    ui.horizontal(|ui| {
        ui.set_enabled(!sim_config.running);
        ui.label("x");
        ui.add(
            DragValue::new(&mut ui_state.size.x)
                .clamp_range(10..=1000)
                .speed(25),
        );
        if ui_state.dim != Dimension::D1 {
            ui.scope(|ui| {
                ui.label("y");
                ui.add(
                    DragValue::new(&mut ui_state.size.y)
                        .clamp_range(10..=1000)
                        .speed(25),
                );
            });
        }
        if ui_state.dim == Dimension::D3 {
            ui.scope(|ui| {
                ui.label("z");
                ui.add(
                    DragValue::new(&mut ui_state.size.z)
                        .clamp_range(10..=1000)
                        .speed(25),
                );
            });
        }
    });
    ui.end_row();

    // Universe population
    ui.label("Noise");
    ui.scope(|ui| {
        ui.set_enabled(!sim_config.running);
        ui.set_visible(ui_state.use_noise);
        ui.set_enabled(ui_state.use_noise);
        ui.add(egui::Slider::new(&mut ui_state.noise, 0.0..=1.0));
    });
    ui.end_row();

    // Automata rules
    ui.label("Rule");
    egui::ComboBox::from_label("")
        .selected_text(&sim_config.rule.name)
        .show_ui(ui, |ui| {
            for rule in &mut rules.rules {
                ui.selectable_value(&mut sim_config.rule, rule.clone(), rule.name.clone());
            }
        });
}
