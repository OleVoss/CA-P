use bevy::prelude::{AppBuilder, IntoSystem, Plugin, SystemSet};

use crate::{systems::simulation_controller::sim_step, DimensionAppState};

pub mod d1;
pub mod d2;
pub mod d3;

pub struct D1Plugin;
impl Plugin for D1Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(DimensionAppState::D1).with_system(d1::d1_enter.system()),
        )
        .add_system_set(
            SystemSet::on_update(DimensionAppState::D1).with_system(d1::d1_update.system()),
        )
        .add_system_set(
            SystemSet::on_update(DimensionAppState::D1).with_system(sim_step.system()),
        )
        .add_system_set(
            SystemSet::on_update(DimensionAppState::D1).with_system(d1::reset_listener.system()),
        )
        .add_system_set(
            SystemSet::on_exit(DimensionAppState::D1).with_system(d1::d1_exit.system()),
        );
    }
}

pub struct D2Plugin;
impl Plugin for D2Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(DimensionAppState::D2).with_system(d2::d2_enter.system()),
            )
            .add_system_set(
                SystemSet::on_update(DimensionAppState::D2).with_system(d2::resize_to_zoom_level.system()),
            )
            .add_system_set(
                SystemSet::on_update(DimensionAppState::D2).with_system(d2::d2_update.system()),
            )
            .add_system_set(
                SystemSet::on_update(DimensionAppState::D2).with_system(sim_step.system()),
            )
            .add_system_set(
                SystemSet::on_update(DimensionAppState::D2).with_system(d2::resize_listener.system()),
            )
            .add_system_set(
                SystemSet::on_update(DimensionAppState::D2).with_system(d2::reset_listener.system()),
            )
            .add_system_set(
                SystemSet::on_exit(DimensionAppState::D2).with_system(d2::d2_exit.system()),
            );
    }
}

pub struct D3Plugin;
impl Plugin for D3Plugin {
    fn build(&self, app: &mut AppBuilder) {}
}
