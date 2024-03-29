use bevy::render::render_resource::{Extent3d, Texture, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;
use bevy::render::{color, texture};
use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_egui::egui::ImageData::Color;
use ca::{CellularAutomata, Shape};

use crate::{
    setup::{DEFAULT_X, DEFAULT_Y},
    ui::{UiState, SIDE_PANEL_WIDTH},
    utils::{simconfig::SimConfig, ResetEvent, ResizeEvent},
    DimensionAppState,
};

pub struct D2UniverseImage(pub Handle<Image>);

pub fn d2_enter(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.insert_resource(CellularAutomata::default());

    let mut temp_img = Image::new(
        Extent3d {
            width: DEFAULT_X as u32,
            height: DEFAULT_Y as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![255; DEFAULT_X * DEFAULT_Y * 4],
        TextureFormat::Rgba8Unorm,
    );
    temp_img.sampler_descriptor = ImageSampler::nearest();

    let image = images.add(temp_img);
    commands.insert_resource(D2UniverseImage(image.clone()));

    // create material
    let sprite = Sprite {
        custom_size: Some(Vec2::new(300., 300.)),
        ..Default::default()
    };

    // spawn sprite
    commands.spawn_bundle(SpriteBundle {
        sprite,
        transform: Transform::from_xyz(SIDE_PANEL_WIDTH / 2., 0., 0.),
        texture: image,
        ..Default::default()
    });
}

pub fn d2_update(
    mut ca: ResMut<CellularAutomata>,
    universe_texture: Res<D2UniverseImage>,
    mut images: ResMut<Assets<Image>>,
    sim_config: Res<SimConfig>,
) {
    if let Some(texture) = images.get_mut(&universe_texture.0) {
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

    commands.remove_resource::<D2UniverseImage>();
}
// utils
pub fn resize_to_zoom_level(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Sprite, &mut Transform)>,
) {
    // TODO zoom to cursor
    if let Some(event) = mouse_wheel_events.iter().last() {
        info!("Tried to zoom on sprite");
        for (mut _sprite, mut transform) in query.iter_mut() {
            info!("{:#?}", transform);
            transform.scale.x = (transform.scale.x + (transform.scale.x * 0.25 * event.y)).min(50.);
            transform.scale.y = (transform.scale.y + (transform.scale.y * 0.25 * event.y)).min(50.);
        }
    }
}

pub fn resize_listener(
    mut events: EventReader<ResizeEvent>,
    sim_config: Res<SimConfig>,
    universe_texture: Res<D2UniverseImage>,
    mut images: ResMut<Assets<Image>>,
    mut ca: ResMut<CellularAutomata>,
    mut query: Query<&mut Sprite>,
) {
    if let Some(_resize_event) = events.iter().last() {
        let image = images.get_mut(&universe_texture.0.clone()).unwrap();

        image.data.clear();
        image.resize(Extent3d {
            width: sim_config.size.x as u32,
            height: sim_config.size.y as u32,
            depth_or_array_layers: 1,
        });
        // alpha value has to be set to 255; hence the repopulation; rgb values are reset in update loop
        image.data = vec![255; (sim_config.size.x * sim_config.size.y * 4.) as usize];

        ca.world = vec![0; sim_config.size.x as usize * sim_config.size.y as usize];
        ca.shape = Shape::new(sim_config.size.x as i32, sim_config.size.y as i32, 0);

        // TODO: not sure if that was important; works without it tho
        // if let Ok(mut sprite) = query.get_single_mut() {
        //     if let Some(mut size) = sprite.custom_size {
        //         size.x = sim_config.size.x;
        //         size.y = sim_config.size.y;
        //     }
        // }
    }
}

#[allow(clippy::erasing_op)]
pub fn reset_listener(mut events: EventReader<ResetEvent>, mut ca: ResMut<CellularAutomata>) {
    if let Some(_reset_event) = events.iter().last() {
        ca.world = ca.world.iter_mut().map(|c| *c * 0u8).collect();
    }
}
