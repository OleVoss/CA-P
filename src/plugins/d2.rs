use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    render::texture::{self, Extent3d},
};
use ca::{CellularAutomata, Shape};

use crate::{DimensionAppState, setup::{DEFAULT_X, DEFAULT_Y}, ui::{UiState, SIDE_PANEL_WIDTH}, utils::{ResetEvent, ResizeEvent, simconfig::SimConfig}};

pub struct D2UniverseTexture(pub Handle<Texture>);

pub fn d2_enter(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    commands.insert_resource(CellularAutomata::default());

    let texture = textures.add(Texture::new(
        Extent3d::new(DEFAULT_X as u32, DEFAULT_Y as u32, 1),
        texture::TextureDimension::D2,
        vec![255; DEFAULT_X * DEFAULT_Y * 4],
        texture::TextureFormat::Rgba8Unorm,
    ));
    commands.insert_resource(D2UniverseTexture(texture.clone()));

    // create material
    let material = materials.add(ColorMaterial::texture(texture));

    // spawn sprite
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(300., 300.)),
        material,
        transform: Transform::from_xyz(SIDE_PANEL_WIDTH / 2., 0., 0.),
        ..Default::default()
    });
}

pub fn d2_update(
    mut ca: ResMut<CellularAutomata>,
    universe_texture: Res<D2UniverseTexture>,
    mut textures: ResMut<Assets<Texture>>,
    sim_config: Res<SimConfig>,
) {
    if let Some(texture) = textures.get_mut(&universe_texture.0) {
        for (state, pixel) in ca.world.iter().zip(texture.data.chunks_mut(4)) {
            if state == &1 {
                pixel[0] = 255; // R
                pixel[1] = 255; // G
                pixel[2] = 255; // B
            } else {
                pixel[0] = 0; // R
                pixel[1] = 0; // G
                pixel[2] = 0; // B
            }
        }
    }
    ca.rule = sim_config.rule.clone();
}

pub fn d2_exit(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    info!("exiting d2");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<D2UniverseTexture>();
}
// utils
pub fn resize_to_zoom_level(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Sprite, &mut Transform)>,
) {
    // TODO zoom to cursor
    if let Some(event) = mouse_wheel_events.iter().last() {
        for (mut sprite, mut _transform) in query.iter_mut() {
            sprite.size.x = (sprite.size.x + (sprite.size.x * 0.25 * event.y)).max(50.);
            sprite.size.y = (sprite.size.y + (sprite.size.y * 0.25 * event.y)).max(50.);
        }
    }
}

pub fn resize_listener(
    mut events: EventReader<ResizeEvent>,
    sim_config: Res<SimConfig>,
    universe_texture: Res<D2UniverseTexture>,
    mut textures: ResMut<Assets<Texture>>,
    mut ca: ResMut<CellularAutomata>,
    mut query: Query<&mut Sprite>,
) {
    if let Some(_resize_event) = events.iter().last() {
        let texture = textures.get_mut(universe_texture.0.clone()).unwrap();

        texture.data.clear();
        texture.data.resize(sim_config.size.x as usize * sim_config.size.y as usize * 4, 255);
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
pub fn reset_listener(
    mut events: EventReader<ResetEvent>,
    mut ca: ResMut<CellularAutomata>,
) {
    if let Some(_reset_event) = events.iter().last() {
        ca.world = ca.world.iter_mut().map(|c| *c * 0u8).collect();
    }
}