use num_enum::FromPrimitive;
use serde::{Serialize, Deserialize};

use crate::sprite::Sprite;

pub struct Map {
    pub identifier: MapIdentifier,
    pub tiles: Vec<Vec<u8>>,
    pub sprites: Vec<Vec<Option<Sprite>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, FromPrimitive)]
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
    Unknown = u8::MAX,
}
