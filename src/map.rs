use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use enum_utils::FromStr;
use num_enum::FromPrimitive;
use serde::{Serialize, Deserialize};

use crate::error::SimpleError;
use crate::sprite::SpriteData;

pub struct Map {
    pub identifier: MapIdentifier,
    pub tiles: Vec<Vec<u8>>,
    pub sprites: Vec<Vec<Option<SpriteData>>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self.identifier)?;
        writeln!(f)?;
        for row in &self.tiles {
            writeln!(f, "{}", row.iter().map(|b| format!("{:02}", b)).collect::<Vec<_>>().join(", "))?;
        }
        writeln!(f)?;
        for row in &self.sprites {
            let row = row.iter().map(|sprite| match sprite {
                Some(sprite) => format!("{:012}", sprite.to_string()),
                None => "None        ".to_string(),
            }).collect::<Vec<_>>().join(", ");

            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (identifier, rest) = s.split_once("\n\n").ok_or(SimpleError("Invalid map format"))?;
        let identifier = identifier.parse().map_err(|()| SimpleError("Invalid map identifier"))?;

        let (tiles, sprites) = rest.split_once("\n\n").ok_or(SimpleError("Invalid map format"))?;
        let tiles = tiles.lines().map(|line| line.split(',').map(str::trim).map(u8::from_str).collect()).collect::<Result<_, _>>()?;
        let sprites = sprites.lines().map(|line| line.split(',').map(str::trim).map(|sprite| if sprite == "None" { Ok(None) } else { sprite.parse().map(Some) }).collect()).collect::<Result<_, _>>()?;

        Ok(Map { identifier, tiles, sprites })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, FromPrimitive, FromStr)]
#[repr(u8)]
pub enum MapIdentifier {
    DustShelf = 1,
    ThroneRoom = 2,
    ExplodingThroneRoom = 4,
    CastleRuins = 5,
    NorthMundeman = 6,
    SouthMundeman = 7,
    VerdantCoast = 8,
    OtherworldArena = 9,
    CastleGrounds = 10,
    Sanctuary = 11,
    TheTunnels = 12,
    Glitch = 13,
    Luddershore = 14,
    TheTundra = 15,
    FrozenShore = 16,
    HallowGround = 17,
    SouthernSwamp = 18,
    DragonsLair = 19,
    CorruptedCastle = 20,
    CastleMonillud = 21,
    ThroneRoomConfrontation = 22,
    TheUnderworld = 23,
    Otherworld = 24,
    MoltenCavern = 26,
    TheDungeons = 27,
    ItemShop = 28,
    Convergence = 29,
    TrialOfReality = 30,
    HauntedManse = 35,
    SmugglersRoad = 40,
    SmugglersRuin = 41,
    #[num_enum(default)]
    #[enumeration(skip)]
    Unknown = u8::MAX,
}

const MAP_ORDER: [MapIdentifier; 31] = [
    MapIdentifier::CastleGrounds,
    MapIdentifier::SouthMundeman,
    MapIdentifier::TheTunnels,
    MapIdentifier::SouthernSwamp,
    MapIdentifier::DustShelf,
    MapIdentifier::TheTundra,
    MapIdentifier::HallowGround,
    MapIdentifier::HauntedManse,
    MapIdentifier::NorthMundeman,
    MapIdentifier::MoltenCavern,
    MapIdentifier::FrozenShore,
    MapIdentifier::VerdantCoast,
    MapIdentifier::Luddershore,
    MapIdentifier::CastleMonillud,
    MapIdentifier::TheDungeons,
    MapIdentifier::ItemShop,
    MapIdentifier::Sanctuary,
    MapIdentifier::DragonsLair,
    MapIdentifier::SmugglersRoad,
    MapIdentifier::SmugglersRuin,
    MapIdentifier::Otherworld,
    MapIdentifier::OtherworldArena,
    MapIdentifier::TheUnderworld,
    MapIdentifier::Glitch,
    MapIdentifier::CorruptedCastle,
    MapIdentifier::ThroneRoom,
    MapIdentifier::ThroneRoomConfrontation,
    MapIdentifier::ExplodingThroneRoom,
    MapIdentifier::CastleRuins,
    MapIdentifier::Convergence,
    MapIdentifier::TrialOfReality,
];

impl MapIdentifier {
    fn order_index(&self) -> usize {
        MAP_ORDER.iter().enumerate()
            .find(|(_, identifier)| self == *identifier)
            .map_or(usize::MAX, |(index, _)| index)
    }
}

impl PartialOrd for MapIdentifier {
    fn partial_cmp(&self, other: &MapIdentifier) -> Option<Ordering> {
        self.order_index().partial_cmp(&other.order_index())
    }
}
impl Ord for MapIdentifier {
    fn cmp(&self, other: &MapIdentifier) -> Ordering {
        self.order_index().cmp(&other.order_index())
    }
}

pub fn decode_map(bytes: Vec<u8>) -> Result<Map, Box<dyn Error>> {
    let map_id = bytes[0];
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
            let position = (10963 * map_id as usize + index * 8) % (1 + bit_index);
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

    let identifier = MapIdentifier::from(map_id);

    Ok(Map { identifier, tiles, sprites })
}

pub fn encode_map(map: Map) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(0);

    bytes.push(map.identifier as u8);
    let width = map.tiles[0].len();
    let height = map.tiles.len();
    bytes.push(width as u8);
    bytes.push(height as u8);

    bytes.extend(
        map.tiles.into_iter().flat_map(|row|
            row.into_iter().flat_map(|tile|
                (0..7).map(move |index| tile & (1 << (6 - index)) != 0)
            )
        ).collect::<Vec<_>>().chunks(8).enumerate().map(|(index, bits)| {
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
        })
    );

    bytes.extend(
        map.sprites.into_iter().enumerate().flat_map(|(y, row)|
            row.into_iter().enumerate().filter_map(move |(x, sprite)| sprite.map(|sprite| (x, y, sprite)))
        ).flat_map(|(x, y, sprite)| {
            let mut bytes = vec![sprite.kind, x as u8, y as u8];
            bytes.extend(sprite.extra_bytes);
            bytes
        })
    );

    bytes
}
