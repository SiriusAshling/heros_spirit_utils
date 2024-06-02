use std::collections::{HashMap, HashSet};

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
                            .or_insert(0u8) += 1
                    }
                    Sprite::Enemy(enemy) => {
                        *sprite_stats
                            .entry(map.identifier)
                            .or_insert_with(SpriteStats::default)
                            .enemies
                            .entry(enemy)
                            .or_insert(0u8) += 1
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
            .map(|collectible| format!("{:?}", collectible))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let collectibles_stats = sprite_stats
        .iter()
        .map(|(map, stats)| {
            format!(
                "{}, {}",
                map::map_name(*map),
                all_collectibles
                    .iter()
                    .map(|collectible| stats
                        .collectibles
                        .get(collectible)
                        .cloned()
                        .unwrap_or(0)
                        .to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let enemies_header = format!(
        ", {}",
        all_enemies
            .iter()
            .map(|enemy| format!("{:?}", enemy))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let enemies_stats = sprite_stats
        .iter()
        .map(|(map, stats)| {
            format!(
                "{}, {}",
                map::map_name(*map),
                all_enemies
                    .iter()
                    .map(|enemy| stats.enemies.get(enemy).cloned().unwrap_or(0).to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let collectibles_stats = format!("{}\n{}", collectibles_header, collectibles_stats);
    let enemies_stats = format!("{}\n{}", enemies_header, enemies_stats);

    util::write("rom_files/Maps/stats/collectibles.csv", collectibles_stats)?;
    util::write("rom_files/Maps/stats/enemies.csv", enemies_stats)?;

    Ok(())
}
