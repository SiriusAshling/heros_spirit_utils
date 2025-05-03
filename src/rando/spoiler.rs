use std::collections::HashMap;

use serde::Serialize;

use crate::map::Gear;

use super::id::Id;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Spoiler {
    pub seed: String,
    pub gear: HashMap<Gear, Id>,
}

impl Spoiler {
    pub fn new(seed: String) -> Self {
        Self {
            seed,
            gear: HashMap::default(),
        }
    }
}
