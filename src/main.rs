mod ui;
use ui::ui_setup;

mod utils;
mod setup;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use utils::simconfig::SimConfig;

fn main() {

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        .add_startup_system(setup::setup.system())
        .add_system(ui_setup.system())

        .insert_resource(WindowDescriptor {
            title: "Cellular Automata - Playground".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .init_resource::<SimConfig>()
    .run();
}