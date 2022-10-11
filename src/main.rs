mod setup;
mod ui;
mod utils;

use crate::ui::draw_ui;
use crate::utils::simconfig::SimConfig;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // systems
        .add_startup_system(setup::setup)
        .add_system(draw_ui)
        // utils
        .init_resource::<SimConfig>()
        .insert_resource(WindowDescriptor {
            title: "Cellular Automaton - Playground".to_string(),
            width: 500.,
            height: 500.,
            ..Default::default()
        })
        .run();
}
