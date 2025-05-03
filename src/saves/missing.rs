use std::collections::HashMap;

use crate::map::{Collectible, Door, Map, Sprite};
use crate::{helpers, Result};

use super::SavePretty;

impl SavePretty {
    pub fn check(&self, name: &str, maps: &[Map]) -> Result<()> {
        let mut missing = HashMap::<_, Vec<_>>::new();

        let ids = relevant_map_ids(name);
        let maps = maps.iter().filter(|map| ids.contains(&map.identifier));

        for map in maps {
            for (x, y, sprite) in map.sprites_with_positions() {
                let sprite = Sprite::from(sprite.kind);
                if is_relevant_sprite(sprite) {
                    let id = map.identifier;
                    let flag = format!("{id}.{x}.{y}");
                    let collected = self.flags.get(&flag).copied().unwrap_or_default();
                    if !collected {
                        missing.entry(format!("{sprite:?}")).or_default().push(flag);
                    }
                }
            }
        }

        let out = serde_json::to_string_pretty(&missing)?;
        helpers::write(format!("completion/{name}_missing.json"), out)?;

        Ok(())
    }
}

fn relevant_map_ids(name: &str) -> &[u8] {
    match name {
        "bunny" => &[Map::HEROS_SPRINT],
        "hcp" => &[
            Map::HHM_CASTLE_GROUNDS,
            Map::HHM_CASTLE_MONILLUD,
            Map::HHM_STRANGE_AREA,
            Map::HHM_THE_UNDERWORLD,
        ],
        "savedata" | "savedatb" | "savedatc" => &[
            Map::DUST_SHELF,
            Map::NORTH_MUNDEMAN,
            Map::SOUTH_MUNDEMAN,
            Map::VERDANT_COAST,
            Map::OTHERWORLD_ARENA,
            Map::CASTLE_GROUNDS,
            Map::SANCTUARY,
            Map::THE_TUNNELS,
            Map::GLITCH,
            Map::LUDDERSHORE,
            Map::THE_TUNDRA,
            Map::FROZEN_SHORE,
            Map::HALLOW_GROUND,
            Map::SOUTHERN_SWAMP,
            Map::DRAGONS_LAIR,
            Map::CORRUPTED_CASTLE,
            Map::CASTLE_MONILLUD,
            Map::THE_UNDERWORLD,
            Map::OTHERWORLD,
            Map::MOLTEN_CAVERN,
            Map::THE_DUNGEONS,
            Map::ITEM_SHOP,
            Map::CONVERGENCE,
            Map::TRIAL_OF_REALITY,
            Map::FALLEN_WORLD,
            Map::ROAD_TO_HELL,
            Map::HAUNTED_MANSE,
            Map::MOONWELL,
            Map::BETWEEN_WORLDS,
            Map::SMUGGLERS_ROAD,
            Map::SMUGGLERS_RUIN,
        ],
        _ => panic!("unknown save name {name}"),
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
