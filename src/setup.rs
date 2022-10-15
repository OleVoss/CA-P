use crate::{
    ui::{UiState, SIDE_PANEL_WIDTH},
    utils::RuleStorage,
};
use bevy::prelude::*;
use bevy::render::render_resource::FilterMode;
use bevy::render::texture::ImageSettings;
use ca::CellularAutomata;

pub const DEFAULT_X: usize = 10;
pub const DEFAULT_Y: usize = 10;
pub const DEFAULT_Z: usize = 0;

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    // ui state
    commands.insert_resource(UiState::default());
    commands.insert_resource(CellularAutomata::default());
    commands.insert_resource(ImageSettings::default_nearest());
    commands.insert_resource(FilterMode::Linear);

    // crate rules
    let storage_string = include_str!("../assets/rules.ron");
    let storage: RuleStorage = ron::from_str(storage_string).unwrap();
    commands.insert_resource(storage);
}
