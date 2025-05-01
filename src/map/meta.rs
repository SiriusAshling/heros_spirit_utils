use std::collections::HashMap;

use indexmap::IndexMap;
use serde::Deserialize;

use crate::{rom::RomReader, Result};

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MapColors {
    pub map_colors: IndexMap<String, IndexMap<String, [u8; 4]>>,
}

impl MapColors {
    pub fn parse(rom: &mut RomReader) -> Result<Self> {
        let index = rom
            .index
            .map_colors
            .ok_or("no Maps/Metadata/MapColors.json in ROM")?;
        let file = rom.archive.by_index(index)?;
        let map_colors: MapColors = serde_json::from_reader(file)?;
        Ok(map_colors)
    }
}

#[derive(Debug, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct MapMeta {
    pub is_bloodmoon_allowed: bool,
    pub is_eclipse_allowed: bool,
    pub is_mirror_allowed: bool,
    pub is_night_allowed: bool,
    pub is_chase_boss_map: bool,
    pub is_snake_boss_map: bool,
    pub is_witch_boss_map: bool,
    pub has_glitch_enemies: bool,
    pub has_glitch_music: bool,
    pub has_ngp_music: bool,
    pub has_ngpp_music: bool,
    pub has_possum_music: bool,
    pub has_glitch_colors: bool,
    pub has_ngp_colors: bool,
    pub has_ngpp_colors: bool,
    pub music: usize,
    pub music_alt: usize,
    pub music_night: usize,
    pub music_night_alt: usize,
    pub colors: usize,
    pub colors_alt: usize,
    pub colors_night: usize,
    pub colors_night_alt: usize,
    pub possum_x: i32,
    pub possum_y: i32,
    pub night_swords: u8,
}

impl MapMeta {
    pub fn parse_all(rom: &mut RomReader) -> Result<HashMap<usize, Self>> {
        rom.index
            .map_meta
            .iter()
            .map(|index: &usize| {
                let file = rom.archive.by_index(*index)?;
                let name = file.name();
                let id = name
                    .strip_prefix("Maps/Metadata/map")
                    .and_then(|name| name.strip_suffix(".json"))
                    .ok_or_else(|| format!("invalid map meta file \"{name}\""))?;
                let id = id.parse().map_err(|err| {
                    format!("invalid map meta identifier \"{id}\" in \"{name}\": {err}")
                })?;

                let map_meta = serde_json::from_reader(file)?;

                Ok((id, map_meta))
            })
            .collect()
    }
}
