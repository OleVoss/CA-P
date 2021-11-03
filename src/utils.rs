pub mod simconfig;

use ca::Rule;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RuleStorage {
    pub rules: Vec<Rule>,
}