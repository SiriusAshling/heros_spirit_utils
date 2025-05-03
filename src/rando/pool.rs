use std::ops::{Deref, DerefMut};

use rand_pcg::Pcg64Mcg;

use crate::{
    helpers::RemoveRandom,
    map::{Map, Sprite},
    Result,
};

use super::logic::Logic;

#[derive(Default)]
pub struct Pool {
    items: Vec<Sprite>,
}

impl Pool {
    pub fn new(logic: &Logic, maps: &[Map]) -> Self {
        let mut items = vec![];

        for item in logic.items() {
            let sprite = item.expect_sprite(maps).kind.into();

            debug_assert!(
                matches!(sprite, Sprite::Collectible(_) | Sprite::Gear(_)),
                "attempted to add {sprite:?} to pool"
            );

            items.push(sprite);
        }

        Self { items }
    }

    pub fn contains_all(&self, items: &[Sprite]) -> bool {
        let mut iter = self.iter();

        for item in items {
            if !iter.any(|i| i == item) {
                return false;
            }
        }

        true
    }

    pub fn choose_remove_filler(&mut self, rng: &mut Pcg64Mcg) -> Result<Sprite> {
        let mut skipped = vec![];

        while !self.is_empty() {
            let sprite = self.choose_remove(rng);

            if let Sprite::Gear(_) = sprite {
                skipped.push(sprite);
            } else {
                self.append(&mut skipped);
                return Ok(sprite);
            }
        }

        Err("failed to choose filler item")?
    }
}

impl Deref for Pool {
    type Target = Vec<Sprite>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}
