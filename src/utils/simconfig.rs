use bevy::math::Vec3;
use ca::{Dimension, Rule};

use crate::setup::{DEFAULT_X, DEFAULT_Y};

pub struct SimConfig {
    pub noise: f32,
    pub rule: Rule,
    pub max_steps: i32,
    pub running: bool,
    pub paused: bool,
    pub step: i32,
    pub use_noise: bool,
    pub dimension: Dimension,
    pub size: Vec3,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            noise: 0.0,
            rule: Rule {
                dimension: Dimension::D2,
                name: "Game of Life".to_string(),
                birth: vec![3],
                survival: vec![2, 3],
            },
            max_steps: 5,
            running: false,
            paused: false,
            step: 0,
            use_noise: false,
            dimension: Dimension::D2,
            size: Vec3::new(DEFAULT_X as f32, DEFAULT_Y as f32, 0.),
        }
    }
}
