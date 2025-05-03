use std::ops::{Deref, DerefMut};

use crate::map::{Map, SpriteData};

use super::id::Id;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Seed {
    pub placements: Vec<(Id, SpriteData)>,
}

impl Seed {
    pub fn apply(self, maps: &mut [Map]) {
        for (location, sprite) in self.placements {
            *location.expect_sprite_mut(maps) = sprite;
        }
    }
}

impl Deref for Seed {
    type Target = Vec<(Id, SpriteData)>;

    fn deref(&self) -> &Self::Target {
        &self.placements
    }
}

impl DerefMut for Seed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.placements
    }
}
