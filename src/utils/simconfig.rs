use ca::{Dimension, Rule};

use crate::setup::{DEFAULT_X, DEFAULT_Y};

pub struct SimConfig {
    pub noise: f32,
    pub rule: Rule,
    pub max_steps: i32,
    pub started: bool,
    pub running: bool,
    pub paused: bool,
    pub step: i32,
    pub use_noise: bool,
    pub dimension: Dimension,
    pub height: i32,
    pub width: i32,
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
            started: false,
            running: false,
            paused: false,
            step: 0,
            use_noise: false,
            dimension: Dimension::D2,
            height: DEFAULT_Y as i32,
            width: DEFAULT_X as i32,
        }
    }
}