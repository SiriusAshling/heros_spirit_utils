use std::{collections::{HashMap, HashSet}, path::Path, fs, error::Error};

use crate::{map::Map, sprite::Sprite, util};

pub fn map_stats<P: AsRef<Path>>(path: P, maps: &[Map]) -> Result<(), Box<dyn Error>> {
    let mut sprite_stats = HashMap::new();

    for map in maps {
        for row in &map.sprites {
            for sprite in row {
                if let Some(sprite) = sprite {
                    if let Sprite::Collectible(collectible) = sprite {
                        *sprite_stats.entry(map.identifier).or_insert_with(HashMap::new).entry(collectible).or_insert(0u8) += 1;
                    }
                }
            }
        }
    }

    let all_collectibles = sprite_stats.iter()
        .map(|(_, collectible_map)| collectible_map.keys().cloned().collect::<HashSet<_>>())
        .fold(HashSet::new(), |acc, other| acc.union(&other).cloned().collect::<HashSet<_>>());

    let header = format!(", {}",
        all_collectibles.iter()
            .map(|collectible| format!("{:?}", collectible))
            .collect::<Vec<_>>().join(", ")
    );
    let sprite_stats = sprite_stats.into_iter().map(|(map, amounts_by_collectible)|
        format!("{:?}, {}",
            map,
            all_collectibles.iter()
                .map(|collectible| amounts_by_collectible.get(collectible).cloned().unwrap_or(0).to_string())
                .collect::<Vec<_>>().join(", ")
        )
    ).collect::<Vec<_>>().join("\n");

    let stats = format!("{}\n{}", header, sprite_stats);

    let mut path = path.as_ref().to_owned();
    util::ensure_dir(&path)?;

    path.push("collectibles.csv");

    fs::write(path, stats)?;

    Ok(())
}
