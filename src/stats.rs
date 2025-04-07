use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::map::{self, Map};
use crate::sprite::{Collectible, Enemy, Sprite};
use crate::{util, Result};

#[derive(Default)]
struct SpriteStats {
    collectibles: HashMap<Collectible, u8>,
    enemies: HashMap<Enemy, u8>,
}

pub fn map_stats(maps: &[Map]) -> Result<()> {
    let mut sprite_stats = HashMap::new();

    for map in maps {
        for row in &map.sprites {
            for sprite in row.iter().flatten() {
                match sprite.kind.into() {
                    Sprite::Collectible(collectible) => {
                        *sprite_stats
                            .entry(map.identifier)
                            .or_insert_with(SpriteStats::default)
                            .collectibles
                            .entry(collectible)
                            .or_insert(0u8) += 1;
                    }
                    Sprite::Enemy(enemy) => {
                        *sprite_stats
                            .entry(map.identifier)
                            .or_insert_with(SpriteStats::default)
                            .enemies
                            .entry(enemy)
                            .or_insert(0u8) += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    let mut all_collectibles = HashSet::<Collectible>::new();
    let mut all_enemies = HashSet::<Enemy>::new();
    for stats in sprite_stats.values() {
        all_collectibles.extend(stats.collectibles.keys());
        all_enemies.extend(stats.enemies.keys());
    }
    let mut all_collectibles = Vec::from_iter(all_collectibles);
    all_collectibles.sort_unstable();
    let mut all_enemies = Vec::from_iter(all_enemies);
    all_enemies.sort_unstable();

    let mut sprite_stats = Vec::from_iter(sprite_stats);
    sprite_stats.sort_unstable_by_key(|(map, _)| *map);

    let collectibles_header = format!(
        ", {}",
        all_collectibles
            .iter()
            .format_with(", ", |collectible, f| f(&format_args!("{collectible:?}")))
    );
    let collectibles_stats = sprite_stats.iter().format_with("\n", |(map, stats), f| {
        f(&format_args!(
            "{}, {}",
            map::map_name(*map),
            all_collectibles
                .iter()
                .map(|collectible| stats.collectibles.get(collectible).copied().unwrap_or(0))
                .format(", ")
        ))
    });
    let enemies_header = format!(
        ", {}",
        all_enemies
            .iter()
            .format_with(", ", |enemy, f| f(&format_args!("{enemy:?}")))
    );
    let enemies_stats = sprite_stats.iter().format_with("\n", |(map, stats), f| {
        f(&format_args!(
            "{}, {}",
            map::map_name(*map),
            all_enemies
                .iter()
                .map(|enemy| stats.enemies.get(enemy).copied().unwrap_or(0))
                .format(", ")
        ))
    });

    let collectibles_stats = format!("{collectibles_header}\n{collectibles_stats}");
    let enemies_stats = format!("{enemies_header}\n{enemies_stats}");

    util::write("rom_files/Maps/stats/collectibles.csv", collectibles_stats)?;
    util::write("rom_files/Maps/stats/enemies.csv", enemies_stats)?;

    Ok(())
}
