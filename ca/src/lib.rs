use std::iter::once;
use serde::Deserialize;

pub mod processor;
mod stepper;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Dimension {
    D1,
    D2,
    D3,
}

pub struct CellularAutomata {
    pub dimension: Dimension,
    pub rule: Rule,
    pub world: Vec<u8>,
    pub shape: Shape,
}

impl Default for CellularAutomata {
    fn default() -> Self {
        Self {
            dimension: Dimension::D2,
            rule: Rule::default(),
            world: vec![0; 2500],
            shape: Shape::new(50, 50, 1)
        }
    }
}

// Builder
impl CellularAutomata {
    pub fn with_dimension(mut self, dimension: Dimension) -> Self {
        self.dimension = dimension;
        self
    }

    pub fn with_rule(mut self, rule: Rule) -> Self {
        self.rule = rule;
        self
    }

    pub fn with_shape(mut self, size: Shape) -> Self {
        self.shape = size;
        self.reset_world();
        self
    }
}

// ca
impl CellularAutomata {
    pub fn reset_world(&mut self) {
        let cell_count: i32 = self.shape.iter().filter(|axis| *axis != 0).product();
        self.world = vec![0; cell_count as usize];
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Rule {
    pub name: String,
    pub dimension: Dimension,
    pub birth: Vec<u8>,
    pub survival: Vec<u8>,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            name: "Game of Life".to_string(),
            dimension: Dimension::D2,
            birth: vec![3],
            survival: vec![2, 3],
        }
    }
}

impl Rule {
    pub fn new(name: String, dimension: Dimension, birth: Vec<u8>,survival: Vec<u8>) -> Self {
        Self {
            name,
            dimension,
            birth,
            survival,
        }
    }
}

pub struct Shape {
    pub x: i32, 
    pub y: i32, 
    pub z: i32,
}

impl Shape {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn iter(&self) -> impl Iterator<Item= i32> {
        once(self.x).chain(once(self.y)).chain(once(self.z))
    }
}