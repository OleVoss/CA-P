use bevy::{
    prelude::*,
    render::texture::{self, Extent3d},
};
use ca::CellularAutomata;
use crate::{ui::{SIDE_PANEL_WIDTH, UiState}, utils::RuleStorage};

pub const DEFAULT_X: usize = 50;
pub const DEFAULT_Y: usize = 50;
pub const DEFAULT_Z: usize = 50;

pub struct WorldTexture(pub Handle<Texture>);

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // ui state
    commands.insert_resource(UiState::default());

    // create texture
    let texture = textures.add(Texture::new(
        Extent3d::new(DEFAULT_X as u32, DEFAULT_Y as u32, 1),
        texture::TextureDimension::D2,
        vec![255; DEFAULT_X * DEFAULT_Y * 4],
        texture::TextureFormat::Rgba8Unorm,
    ));
    commands.insert_resource(WorldTexture(texture.clone()));

    // create material
    let material = materials.add(ColorMaterial::texture(texture));

    // spawn sprite
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(DEFAULT_Y as f32, DEFAULT_X as f32)),
        material,
        transform: Transform::from_xyz(SIDE_PANEL_WIDTH / 2., 0., 0.),
        ..Default::default()
    });

    // create CA
    commands.insert_resource(
        CellularAutomata::default()
    );

    // crate rules
    let storage_string = include_str!("../assets/rules.ron");
    let storage: RuleStorage = ron::from_str(storage_string).unwrap();
    commands.insert_resource(storage);
}