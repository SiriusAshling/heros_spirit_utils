use std::error::Error;

use crate::sprite::SpriteData;

pub struct Map {
    pub identifier: u8,
    pub tiles: Vec<Vec<u8>>,
    pub sprites: Vec<Vec<Option<SpriteData>>>,
}

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
pub const HAUNTED_MANSE: u8 = 35;
pub const SMUGGLERS_ROAD: u8 = 40;
pub const SMUGGLERS_RUIN: u8 = 41;

pub fn map_name(map: u8) -> &'static str {
    match map {
        DUST_SHELF => "DustShelf",
        THRONE_ROOM => "ThroneRoom",
        EXPLODING_THRONE_ROOM => "ExplodingThroneRoom",
        CASTLE_RUINS => "CastleRuins",
        NORTH_MUNDEMAN => "NorthMundeman",
        SOUTH_MUNDEMAN => "SouthMundeman",
        VERDANT_COAST => "VerdantCoast",
        OTHERWORLD_ARENA => "OtherworldArena",
        CASTLE_GROUNDS => "CastleGrounds",
        SANCTUARY => "Sanctuary",
        THE_TUNNELS => "TheTunnels",
        GLITCH => "Glitch",
        LUDDERSHORE => "Luddershore",
        THE_TUNDRA => "TheTundra",
        FROZEN_SHORE => "FrozenShore",
        HALLOW_GROUND => "HallowGround",
        SOUTHERN_SWAMP => "SouthernSwamp",
        DRAGONS_LAIR => "DragonsLair",
        CORRUPTED_CASTLE => "CorruptedCastle",
        CASTLE_MONILLUD => "CastleMonillud",
        THRONE_ROOM_CONFRONTATION => "ThroneRoomConfrontation",
        THE_UNDERWORLD => "TheUnderworld",
        OTHERWORLD => "Otherworld",
        MOLTEN_CAVERN => "MoltenCavern",
        THE_DUNGEONS => "TheDungeons",
        ITEM_SHOP => "ItemShop",
        CONVERGENCE => "Convergence",
        TRIAL_OF_REALITY => "TrialOfReality",
        HAUNTED_MANSE => "HauntedManse",
        SMUGGLERS_ROAD => "SmugglersRoad",
        SMUGGLERS_RUIN => "SmugglersRuin",
        _ => "Unknown",
    }
}

const MAP_ORDER: [u8; 31] = [
    CASTLE_GROUNDS,
    SOUTH_MUNDEMAN,
    THE_TUNNELS,
    SOUTHERN_SWAMP,
    DUST_SHELF,
    THE_TUNDRA,
    HALLOW_GROUND,
    HAUNTED_MANSE,
    NORTH_MUNDEMAN,
    MOLTEN_CAVERN,
    FROZEN_SHORE,
    VERDANT_COAST,
    LUDDERSHORE,
    CASTLE_MONILLUD,
    THE_DUNGEONS,
    ITEM_SHOP,
    SANCTUARY,
    DRAGONS_LAIR,
    SMUGGLERS_ROAD,
    SMUGGLERS_RUIN,
    OTHERWORLD,
    OTHERWORLD_ARENA,
    THE_UNDERWORLD,
    GLITCH,
    CORRUPTED_CASTLE,
    THRONE_ROOM,
    THRONE_ROOM_CONFRONTATION,
    EXPLODING_THRONE_ROOM,
    CASTLE_RUINS,
    CONVERGENCE,
    TRIAL_OF_REALITY,
];

pub fn map_order_index(map: u8) -> usize {
    MAP_ORDER
        .iter()
        .enumerate()
        .find(|(_, identifier)| map == **identifier)
        .map_or(usize::MAX, |(index, _)| index)
}

pub fn decode_map(bytes: Vec<u8>) -> Result<Map, Box<dyn Error>> {
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

pub fn encode_map(map: Map) -> Vec<u8> {
    let tile_byte_count = map.tiles.iter().flatten().count();
    let sprite_byte_count = map
        .sprites
        .iter()
        .flatten()
        .filter_map(|sprite| sprite.as_ref())
        .map(|sprite| 1 + sprite.extra_bytes.len())
        .sum::<usize>();
    let mut bytes = Vec::with_capacity(3 + tile_byte_count + sprite_byte_count);

    bytes.push(map.identifier);
    let width = map.tiles[0].len();
    let height = map.tiles.len();
    bytes.push(width as u8);
    bytes.push(height as u8);

    bytes.extend(
        map.tiles
            .into_iter()
            .flat_map(|row| {
                row.into_iter()
                    .flat_map(|tile| (0..7).map(move |index| tile & (1 << (6 - index)) != 0))
            })
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
                    byte |= (bits.remove(position) as u8) << bit_index;
                }
                byte
            }),
    );

    bytes.extend(
        map.sprites
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(move |(x, sprite)| sprite.map(|sprite| (x, y, sprite)))
            })
            .flat_map(|(x, y, sprite)| {
                let mut bytes = vec![sprite.kind, x as u8, y as u8];
                bytes.extend(sprite.extra_bytes);
                bytes
            }),
    );

    bytes
}
