use std::collections::HashMap;

use crate::map::{self, Map};
use crate::savedata::SavePretty;
use crate::sprite::{Collectible, Door, Sprite};
use crate::{util, Result};

pub fn check(name: &str, save: Option<SavePretty>, maps: &[Map]) -> Result<()> {
    if let Some(save) = save {
        let mut missing = HashMap::<_, Vec<_>>::new();

        let ids = relevant_map_ids(name);
        let maps = maps.iter().filter(|map| ids.contains(&map.identifier));

        for map in maps {
            for (y, row) in map.sprites.iter().enumerate() {
                for (x, sprite) in row.iter().enumerate() {
                    if let Some(sprite) = sprite {
                        let sprite = Sprite::from(sprite.kind);
                        if is_relevant_sprite(sprite) {
                            let id = map.identifier;
                            let flag = format!("{id}.{x}.{y}");
                            let collected = save.flags.get(&flag).copied().unwrap_or_default();
                            if !collected {
                                missing.entry(format!("{sprite:?}")).or_default().push(flag);
                            }
                        }
                    }
                }
            }
        }

        let out = serde_json::to_string_pretty(&missing)?;
        util::write(format!("completion/{name}_missing.json"), out)?;
    }

    Ok(())
}

fn relevant_map_ids(name: &str) -> &[u8] {
    match name {
        "bunny" => &[map::HEROS_SPRINT],
        _ => &[
            map::DUST_SHELF,
            map::NORTH_MUNDEMAN,
            map::SOUTH_MUNDEMAN,
            map::VERDANT_COAST,
            map::OTHERWORLD_ARENA,
            map::CASTLE_GROUNDS,
            map::SANCTUARY,
            map::THE_TUNNELS,
            map::GLITCH,
            map::LUDDERSHORE,
            map::THE_TUNDRA,
            map::FROZEN_SHORE,
            map::HALLOW_GROUND,
            map::SOUTHERN_SWAMP,
            map::DRAGONS_LAIR,
            map::CORRUPTED_CASTLE,
            map::CASTLE_MONILLUD,
            map::THE_UNDERWORLD,
            map::OTHERWORLD,
            map::MOLTEN_CAVERN,
            map::THE_DUNGEONS,
            map::ITEM_SHOP,
            map::CONVERGENCE,
            map::TRIAL_OF_REALITY,
            map::FALLEN_WORLD,
            map::ROAD_TO_HELL,
            map::HAUNTED_MANSE,
            map::MOONWELL,
            map::BETWEEN_WORLDS,
            map::SMUGGLERS_ROAD,
            map::SMUGGLERS_RUIN,
        ],
    }
}

fn is_relevant_sprite(sprite: Sprite) -> bool {
    matches!(
        sprite,
        Sprite::Collectible(
            Collectible::GoldKey
                | Collectible::SilverKey
                | Collectible::RedKey
                | Collectible::GreenKey
                | Collectible::BlueKey
                | Collectible::Sword
                | Collectible::PortalStone
                | Collectible::Gem
                | Collectible::Treasure
                | Collectible::ShrineKey
                | Collectible::GemHeart
                | Collectible::TealKey
                | Collectible::PurpleKey
                | Collectible::UnderworldKey
                | Collectible::PossumCoin
        ) | Sprite::Gear(_)
            | Sprite::Door(
                Door::Gold | Door::Silver | Door::Red | Door::Green | Door::Blue | Door::Underworld
            )
    )
}
