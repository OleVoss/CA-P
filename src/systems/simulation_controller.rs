use std::ops::DerefMut;

use bevy::{input::Input, prelude::{KeyCode, Res, ResMut}};
use ca::CellularAutomata;

use crate::{ui::UiState, utils::{simconfig::SimConfig, universe::generate_noise_universe}};

pub fn sim_step(
    mut ca: ResMut<CellularAutomata>,
    ui_state: Res<UiState>,
    mut sim_config: ResMut<SimConfig>,
) {
    if (sim_config.running && !sim_config.paused) || ui_state.step {
        ca::processor::step(ca.deref_mut());
        sim_config.step += 1;
    }
}

pub fn init_noise(
    mut ca: ResMut<CellularAutomata>,
    input: Res<Input<KeyCode>>,
    ui_state: Res<UiState>,
) {
    if input.just_pressed(KeyCode::N) {
        ca.world = generate_noise_universe(ui_state.noise, ui_state.size);
    }
}