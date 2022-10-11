pub mod simconfig;
pub mod universe;

use ca::Rule;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RuleStorage {
    pub rules: Vec<Rule>,
}

pub struct ResizeEvent;
pub struct ResetEvent;

// TODO use generic cleanup system
