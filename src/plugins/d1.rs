use bevy::{
    prelude::*,
    render::texture::{self, Extent3d},
};
use ca::{CellularAutomata, Dimension, Shape};

use crate::{
    plugins::d2::D2UniverseTexture,
    setup::{DEFAULT_X, DEFAULT_Y},
    ui::{UiState, SIDE_PANEL_WIDTH},
    utils::{simconfig::SimConfig, ResetEvent, ResizeEvent},
};

pub struct D1UniverseTexture(pub Handle<Texture>);

pub fn d1_enter(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut sim_config: ResMut<SimConfig>,
    mut ui_state: ResMut<UiState>,
) {
    info!("entering d1");

    sim_config.size = Vec3::new(DEFAULT_X as f32, 1., 0.);
    ui_state.size = Vec3::new(DEFAULT_X as f32, 1., 0.);

    let ca = CellularAutomata::default()
        .with_dimension(Dimension::D1)
        .with_shape(Shape::new(10, 1, 0));

    let mut data = Vec::<u8>::new();
    for _ in 0..DEFAULT_X {
        data.append(&mut vec![0, 0, 0, 255]);
    } 

    commands.insert_resource(ca);

    let texture = textures.add(Texture::new(
        Extent3d::new(DEFAULT_X as u32, 1, 1),
        texture::TextureDimension::D2,
        data,
        texture::TextureFormat::Rgba8Unorm,
    ));

    commands.insert_resource(D1UniverseTexture(texture.clone()));

    // create material
    let material = materials.add(ColorMaterial::texture(texture));

    // spawn sprite
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(300., 30.)),
        material,
        transform: Transform::from_xyz(SIDE_PANEL_WIDTH / 2., 0., 0.),
        ..Default::default()
    });
}
pub fn d1_update(
    mut ca: ResMut<CellularAutomata>,
    universe_texture: Res<D1UniverseTexture>,
    mut textures: ResMut<Assets<Texture>>,
    sim_config: Res<SimConfig>,
    mut query: Query<(&mut Sprite, &mut Transform)>,
) {
    if let Some(texture) = textures.get_mut(&universe_texture.0) {
        if texture.size.height - 1 < sim_config.step as u32 {
            let mut data = ca.world.clone();
            texture
                .data
                .append(&mut data);

            texture.size.height += 1;

            query
                .single_mut()
                .into_iter()
                .for_each(|(mut sprite, mut _transform)| {
                    let ratio = sprite.size.y / ca.shape.y as f32;
                    sprite.size.y += ratio;
                });
        }
    }

    ca.rule = sim_config.rule.clone();
}

pub fn d1_exit() {}

pub fn resize_listener(
    mut events: EventReader<ResizeEvent>,
    sim_config: Res<SimConfig>,
    universe_texture: Res<D1UniverseTexture>,
    mut textures: ResMut<Assets<Texture>>,
    mut ca: ResMut<CellularAutomata>,
    mut query: Query<&mut Sprite>,
) {
    if let Some(_resize_event) = events.iter().last() {
        let texture = textures.get_mut(universe_texture.0.clone()).unwrap();

        texture.data.clear();
        texture.data.resize(
            sim_config.size.x as usize * sim_config.size.y as usize * 4,
            255,
        );
        texture.size = Extent3d::new(sim_config.size.x as u32, sim_config.size.y as u32, 1);

        ca.world = vec![0; sim_config.size.x as usize * sim_config.size.y as usize];
        ca.shape = Shape::new(sim_config.size.x as i32, sim_config.size.y as i32, 0);

        if let Ok(mut sprite) = query.single_mut() {
            sprite.size.x = sim_config.size.x;
            sprite.size.y = sim_config.size.y;
        }
    }
}

#[allow(clippy::erasing_op)]
pub fn reset_listener(mut events: EventReader<ResetEvent>, mut ca: ResMut<CellularAutomata>) {
    if let Some(_reset_event) = events.iter().last() {
        ca.world = ca.world.iter_mut().map(|c| *c * 0u8).collect();
    }
}
