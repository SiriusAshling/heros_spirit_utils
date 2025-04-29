use std::collections::HashMap;

use crate::map::{Collectible, Enemy, Gear, Map, Sprite};

#[derive(Default)]
pub struct SpriteStats {
    pub collectibles: HashMap<Collectible, u8>,
    pub gear: HashMap<Gear, u8>,
    pub enemies: HashMap<Enemy, u8>,
}

impl Map {
    pub fn stats(&self) -> SpriteStats {
        let mut sprite_stats = SpriteStats::default();

        for sprite in self.sprites() {
            match sprite.kind.into() {
                Sprite::Collectible(collectible) => {
                    *sprite_stats.collectibles.entry(collectible).or_default() += 1;
                }
                Sprite::Gear(gear) => {
                    *sprite_stats.gear.entry(gear).or_default() += 1;
                }
                Sprite::Enemy(enemy) => {
                    *sprite_stats.enemies.entry(enemy).or_default() += 1;
                }
                _ => {}
            }
        }

        sprite_stats
    }
}
