use bevy::prelude::{App, IntoSystem, Plugin, SystemSet};

use crate::{systems::simulation_controller::sim_step, DimensionAppState};

pub mod d2;

pub struct D2Plugin;
impl Plugin for D2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(DimensionAppState::D2).with_system(d2::d2_enter))
            .add_system_set(
                SystemSet::on_update(DimensionAppState::D2).with_system(d2::resize_to_zoom_level),
            )
            .add_system_set(SystemSet::on_update(DimensionAppState::D2).with_system(d2::d2_update))
            .add_system_set(SystemSet::on_update(DimensionAppState::D2).with_system(sim_step))
            .add_system_set(
                SystemSet::on_update(DimensionAppState::D2).with_system(d2::resize_listener),
            )
            .add_system_set(
                SystemSet::on_update(DimensionAppState::D2).with_system(d2::reset_listener),
            )
            .add_system_set(SystemSet::on_exit(DimensionAppState::D2).with_system(d2::d2_exit));
    }
}
