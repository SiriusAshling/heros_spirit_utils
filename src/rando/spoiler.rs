use indexmap::IndexMap;
use serde::Serialize;

use crate::map::Gear;

use super::id::Id;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Spoiler {
    pub seed: String,
    #[serde(flatten)]
    pub items: ItemSpoiler,
}

#[derive(Debug, PartialEq, Eq, Serialize, Default)]
pub struct ItemSpoiler {
    pub gear: IndexMap<Gear, Id>,
}
