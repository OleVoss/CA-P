mod plugins;
mod setup;
mod systems;
mod ui;
mod utils;

use crate::plugins::D2Plugin;
use crate::systems::config_controller::config_controlling;
use crate::systems::simulation_controller::init_noise;
use crate::ui::draw_ui;
use crate::utils::simconfig::SimConfig;
use crate::utils::{ResetEvent, ResizeEvent};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DimensionAppState {
    D1,
    D2,
    D3,
}

#[derive(PartialEq, Eq)]
pub enum AppMode {
    Configuration,
    Simulation,
}

pub struct AppModeFlag {
    mode: AppMode,
}

impl Default for AppModeFlag {
    fn default() -> Self {
        Self {
            mode: AppMode::Configuration,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // dimensions
        // todo: add dimension plugins
        .add_plugin(D2Plugin)
        .add_state(DimensionAppState::D2)
        // systems
        .add_startup_system(setup::setup)
        .add_system(draw_ui)
        .add_system(config_controlling)
        .add_system(init_noise)
        .add_event::<ResizeEvent>()
        .add_event::<ResetEvent>()
        // utils
        .init_resource::<SimConfig>()
        .init_resource::<AppModeFlag>()
        .insert_resource(WindowDescriptor {
            title: "Cellular Automaton - Playground".to_string(),
            width: 500.,
            height: 500.,
            ..Default::default()
        })
        .run();
}
