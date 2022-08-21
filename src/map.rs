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
