use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    path::Path,
};

use crate::{
    map::{self, Map},
    sprite::{Collectible, Enemy, Sprite},
    util,
};

#[derive(Default)]
struct SpriteStats {
    collectibles: HashMap<Collectible, u8>,
    enemies: HashMap<Enemy, u8>,
}

pub fn map_stats(path: impl AsRef<Path>, maps: &[Map]) -> Result<(), Box<dyn Error>> {
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
    let mut all_enemies = Vec::from_iter(all_enemies);
    all_enemies.sort_unstable();

    let mut sprite_stats = Vec::from_iter(sprite_stats);
    sprite_stats.sort_unstable_by_key(|(map, _)| map::map_order_index(*map));

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

    let path = path.as_ref();
    util::ensure_dir(&path)?;

    let mut collectibles_path = path.to_owned();
    collectibles_path.push("collectibles.csv");
    fs::write(collectibles_path, collectibles_stats)?;

    let mut enemies_path = path.to_owned();
    enemies_path.push("enemies.csv");
    fs::write(enemies_path, enemies_stats)?;

    Ok(())
}
