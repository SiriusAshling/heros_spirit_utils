mod meta;
mod sprite;
mod stats;
mod tiled;

pub use meta::{MapColors, MapMeta};
pub use sprite::{Collectible, Door, Enemy, Gear, Sprite, SpriteData, Things};

use crate::rom::{self, RomReader};
use crate::Result;

pub struct Map {
    pub identifier: u8,
    pub tiles: Vec<Vec<u8>>,
    pub sprites: Vec<Vec<Option<SpriteData>>>,
}

impl Map {
    pub const DUST_SHELF: u8 = 1;
    pub const THRONE_ROOM: u8 = 2;
    pub const EXPLODING_THRONE_ROOM: u8 = 4;
    pub const CASTLE_RUINS: u8 = 5;
    pub const NORTH_MUNDEMAN: u8 = 6;
    pub const SOUTH_MUNDEMAN: u8 = 7;
    pub const VERDANT_COAST: u8 = 8;
    pub const OTHERWORLD_ARENA: u8 = 9;
    pub const CASTLE_GROUNDS: u8 = 10;
    pub const SANCTUARY: u8 = 11;
    pub const THE_TUNNELS: u8 = 12;
    pub const GLITCH: u8 = 13;
    pub const LUDDERSHORE: u8 = 14;
    pub const THE_TUNDRA: u8 = 15;
    pub const FROZEN_SHORE: u8 = 16;
    pub const HALLOW_GROUND: u8 = 17;
    pub const SOUTHERN_SWAMP: u8 = 18;
    pub const DRAGONS_LAIR: u8 = 19;
    pub const CORRUPTED_CASTLE: u8 = 20;
    pub const CASTLE_MONILLUD: u8 = 21;
    pub const THRONE_ROOM_CONFRONTATION: u8 = 22;
    pub const THE_UNDERWORLD: u8 = 23;
    pub const OTHERWORLD: u8 = 24;
    pub const MOLTEN_CAVERN: u8 = 26;
    pub const THE_DUNGEONS: u8 = 27;
    pub const ITEM_SHOP: u8 = 28;
    pub const CONVERGENCE: u8 = 29;
    pub const TRIAL_OF_REALITY: u8 = 30;
    pub const FALLEN_WORLD: u8 = 33;
    pub const ROAD_TO_HELL: u8 = 34;
    pub const HAUNTED_MANSE: u8 = 35;
    pub const MOONWELL: u8 = 36;
    pub const BETWEEN_WORLDS: u8 = 37;
    pub const HEROS_SPRINT: u8 = 38;
    pub const MUNDEMAN: u8 = 39;
    pub const SMUGGLERS_ROAD: u8 = 40;
    pub const SMUGGLERS_RUIN: u8 = 41;
    pub const HHM_CASTLE_GROUNDS: u8 = 42;
    pub const HHM_CASTLE_MONILLUD: u8 = 43;
    pub const HHM_STRANGE_AREA: u8 = 44;
    pub const HHM_THE_UNDERWORLD: u8 = 45;
    pub const HHM_THRONE_ROOM: u8 = 46;

    pub fn parse_all(rom: &mut RomReader) -> Result<Vec<Self>> {
        rom.index
            .maps
            .iter()
            .map(|index| {
                let bytes = rom::read_by_index(&mut rom.archive, *index)?;
                let map = decode_map(&bytes)?;
                Ok(map)
            })
            .collect()
    }

    pub fn tiles(&self) -> impl Iterator<Item = u8> + use<'_> {
        self.tiles.iter().flatten().copied()
    }

    pub fn tiles_with_positions(&self) -> impl Iterator<Item = (usize, usize, u8)> + use<'_> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| (x, y, *tile)))
    }

    pub fn sprites(&self) -> impl Iterator<Item = &SpriteData> {
        self.sprites.iter().flatten().flatten()
    }

    pub fn sprites_with_positions(&self) -> impl Iterator<Item = (usize, usize, &SpriteData)> {
        self.sprites.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, sprite)| sprite.as_ref().map(|sprite| (x, y, sprite)))
        })
    }
}

pub fn map_name(map: u8) -> &'static str {
    match map {
        Map::DUST_SHELF => "DustShelf",
        Map::THRONE_ROOM => "ThroneRoom",
        Map::EXPLODING_THRONE_ROOM => "ExplodingThroneRoom",
        Map::CASTLE_RUINS => "CastleRuins",
        Map::NORTH_MUNDEMAN => "NorthMundeman",
        Map::SOUTH_MUNDEMAN => "SouthMundeman",
        Map::VERDANT_COAST => "VerdantCoast",
        Map::OTHERWORLD_ARENA => "OtherworldArena",
        Map::CASTLE_GROUNDS => "CastleGrounds",
        Map::SANCTUARY => "Sanctuary",
        Map::THE_TUNNELS => "TheTunnels",
        Map::GLITCH => "Glitch",
        Map::LUDDERSHORE => "Luddershore",
        Map::THE_TUNDRA => "TheTundra",
        Map::FROZEN_SHORE => "FrozenShore",
        Map::HALLOW_GROUND => "HallowGround",
        Map::SOUTHERN_SWAMP => "SouthernSwamp",
        Map::DRAGONS_LAIR => "DragonsLair",
        Map::CORRUPTED_CASTLE => "CorruptedCastle",
        Map::CASTLE_MONILLUD => "CastleMonillud",
        Map::THRONE_ROOM_CONFRONTATION => "ThroneRoomConfrontation",
        Map::THE_UNDERWORLD => "TheUnderworld",
        Map::OTHERWORLD => "Otherworld",
        Map::MOLTEN_CAVERN => "MoltenCavern",
        Map::THE_DUNGEONS => "TheDungeons",
        Map::ITEM_SHOP => "ItemShop",
        Map::CONVERGENCE => "Convergence",
        Map::TRIAL_OF_REALITY => "TrialOfReality",
        Map::FALLEN_WORLD => "FallenWorld",
        Map::ROAD_TO_HELL => "RoadToHell",
        Map::HAUNTED_MANSE => "HauntedManse",
        Map::MOONWELL => "Moonwell",
        Map::BETWEEN_WORLDS => "BetweenWorlds",
        Map::HEROS_SPRINT => "HerosSprint",
        Map::MUNDEMAN => "Mundeman",
        Map::SMUGGLERS_ROAD => "SmugglersRoad",
        Map::SMUGGLERS_RUIN => "SmugglersRuin",
        Map::HHM_CASTLE_GROUNDS => "HaphyCastleGrounds",
        Map::HHM_CASTLE_MONILLUD => "HaphyCastleMonillud",
        Map::HHM_STRANGE_AREA => "HaphyStrangeArea",
        Map::HHM_THE_UNDERWORLD => "HaphyTheUnderworld",
        Map::HHM_THRONE_ROOM => "HaphyThroneRoom",
        _ => "Unknown",
    }
}

fn decode_map(bytes: &[u8]) -> Result<Map> {
    let identifier = bytes[0];
    let width = bytes[1];
    let width_usize = width as usize;
    let height = bytes[2];
    let height_usize = height as usize;

    let tiles_end = 3 + width_usize * height_usize * 7 / 8;
    let tile_bytes = &bytes[3..tiles_end];
    let sprite_data = &bytes[tiles_end..];

    let mut tile_bits = Vec::with_capacity(tile_bytes.len() * 8);

    for (index, byte) in tile_bytes.iter().enumerate() {
        let mut bits = Vec::with_capacity(8);

        for bit_index in 0..8 {
            let bit = byte & (1 << bit_index) != 0;
            let position = (10963 * identifier as usize + index * 8) % (1 + bit_index);
            bits.insert(position, bit);
        }

        tile_bits.append(&mut bits);
    }

    let mut tile_chunks = tile_bits.chunks(7);

    let mut read_tile = || {
        tile_chunks.next().and_then(|tile_bits| {
            if tile_bits.len() < 7 {
                return None;
            }

            let mut tile = 0u8;
            for (bit_index, bit) in tile_bits.iter().enumerate().take(7) {
                if *bit {
                    tile |= 1 << (6 - bit_index);
                }
            }
            Some(tile)
        })
    };

    let mut tiles = Vec::with_capacity(height_usize);
    for _ in 0..height {
        let mut row = Vec::with_capacity(width_usize);

        for _ in 0..width {
            if let Some(tile) = read_tile() {
                row.push(tile);
            }
        }

        tiles.push(row);
    }

    let mut sprites = vec![vec![None; width_usize]; height_usize];
    let mut sprite_index = 0;
    let len = sprite_data.len();
    while sprite_index < len {
        let (x, y, sprite) = SpriteData::read(sprite_data, &mut sprite_index)?;
        sprites[y as usize][x as usize] = Some(sprite);
    }

    Ok(Map {
        identifier,
        tiles,
        sprites,
    })
}

pub fn encode_map(map: &Map) -> Vec<u8> {
    let tile_byte_count = map.tiles().count();
    let sprite_byte_count = map
        .sprites()
        .map(|sprite| 1 + sprite.extra_bytes.len())
        .sum::<usize>();
    let mut bytes = Vec::with_capacity(3 + tile_byte_count + sprite_byte_count);

    bytes.push(map.identifier);
    let width = map.tiles[0].len();
    let height = map.tiles.len();
    bytes.push(width as u8);
    bytes.push(height as u8);

    bytes.extend(
        map.tiles()
            .flat_map(|tile| (0..7).map(move |index| tile & (1 << (6 - index)) != 0))
            .collect::<Vec<_>>()
            .chunks(8)
            .enumerate()
            .map(|(index, bits)| {
                let mut byte = 0;
                let mut bits = bits.to_vec();
                while bits.len() < 8 {
                    bits.push(false);
                }

                for bit_index in (0..8).rev() {
                    let position = (10963 * map.identifier as usize + index * 8) % (1 + bit_index);
                    byte |= (u8::from(bits.remove(position))) << bit_index;
                }
                byte
            }),
    );

    bytes.extend(map.sprites_with_positions().flat_map(|(x, y, sprite)| {
        let mut bytes = vec![sprite.kind, x as u8, y as u8];
        bytes.extend(sprite.extra_bytes.iter().copied());
        bytes
    }));

    bytes
}
