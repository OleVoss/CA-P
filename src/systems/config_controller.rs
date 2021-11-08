use bevy::{input::mouse::MouseWheel, prelude::*};
use ca::Dimension;

use crate::{AppMode, AppModeFlag, DimensionAppState, ui::UiState, utils::{ResetEvent, ResizeEvent, simconfig::SimConfig}};

// System to mirror ui changes to the simulations config.
pub fn config_controlling(
    mut ui_state: ResMut<UiState>,
    mut sim_config: ResMut<SimConfig>,
    mut mode: ResMut<AppModeFlag>,
    mut app_dimension_state: ResMut<State<DimensionAppState>>,
    mut resize_event: EventWriter<ResizeEvent>,
    mut reset_event: EventWriter<ResetEvent>,
) {
    // state handling
    if ui_state.start {
        mode.mode = AppMode::Simulation;
        sim_config.running = true;
        sim_config.paused = false;
    } else if ui_state.step {
        mode.mode = AppMode::Configuration;
        sim_config.running = true;
        sim_config.paused = true;
    } else if ui_state.reset {
        mode.mode = AppMode::Configuration;
        sim_config.running = false;
        sim_config.paused = false;
        reset_event.send(ResetEvent {})
    } else if ui_state.pause {
        sim_config.paused = true;
    }

    if mode.mode == AppMode::Configuration {

        if sim_config.size != ui_state.size {
            sim_config.size = ui_state.size;
            resize_event.send(ResizeEvent{});
            info!("resize to: {}", sim_config.size);
        }

        if ui_state.dim != sim_config.dimension 
            && app_dimension_state.overwrite_set(dimension_to_appstate(ui_state.dim)).is_ok() 
        {
            info!("switching from {:?} to {:?}", sim_config.dimension, ui_state.dim);
            sim_config.dimension = ui_state.dim;
        } else {
            ui_state.dim = sim_config.dimension;
        }
    }
}

fn dimension_to_appstate(dimension: Dimension) -> DimensionAppState {
    match dimension {
        Dimension::D1 => DimensionAppState::D1,
        Dimension::D2 => DimensionAppState::D2,
        Dimension::D3 => DimensionAppState::D3,
    }
}