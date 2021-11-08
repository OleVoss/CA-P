mod ui;
use plugins::{D1Plugin, D2Plugin, D3Plugin};
use systems::{config_controller::{config_controlling}, simulation_controller::init_noise};
use ui::draw_ui;

mod systems;
mod plugins;

mod utils;
mod setup;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use utils::{ResetEvent, ResizeEvent, simconfig::SimConfig};

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
        Self { mode: AppMode::Configuration }
    }
}

fn main() {

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        // dimension plugins
        .add_plugin(D1Plugin)
        .add_plugin(D2Plugin)
        .add_plugin(D3Plugin)

        .add_state(DimensionAppState::D2) // default dimension

        .add_startup_system(setup::setup.system())
        .add_system(draw_ui.system())

        // config events; systems
        .add_system(config_controlling.system())
        .add_system(init_noise.system())
        .add_event::<ResizeEvent>()
        .add_event::<ResetEvent>()

        // utils
        .insert_resource(WindowDescriptor {
            title: "Cellular Automata - Playground".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .init_resource::<SimConfig>()
        .init_resource::<AppModeFlag>()
    .run();
}