use bevy::prelude::*;
use ca::{CellularAutomata, Dimension, Shape};

use crate::{ui::UiState, utils::simconfig::SimConfig};

pub fn d3_enter(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
) {
    commands.insert_resource(CellularAutomata::default()
        .with_dimension(Dimension::D3)
        .with_shape(Shape::new(10, 10, 10)));

    ui_state.size = Vec3::new(10., 10., 10.);

}

pub fn d3_update() {
}