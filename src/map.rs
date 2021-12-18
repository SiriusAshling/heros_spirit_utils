use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Map {
    DustShelf,
    ThroneRoom,
    ExplodingThroneRoom,
    CastleRuins,
    NorthMundeman,
    SouthMundeman,
    VerdantCoast,
    OtherworldArena,
    CastleGrounds,
    Sanctuary,
    TheTunnels,
    Glitch,
    Luddershore,
    TheTundra,
    FrozenShore,
    HallowGround,
    SouthernSwamp,
    DragonsLair,
    CorruptedCastle,
    CastleMonillud,
    ThroneRoomConfrontation,
    TheUnderworld,
    Otherworld,
    MoltenCavern,
    TheDungeons,
    ItemShop,
    Convergence,
    TrialOfReality,
    HauntedManse,
    SmugglersRoad,
    SmugglersRuin,
    Unknown,
}

impl From<u8> for Map {
    fn from(id: u8) -> Self {
        match id {
            1 => Map::DustShelf,
            2 => Map::ThroneRoom,
            4 => Map::ExplodingThroneRoom,
            5 => Map::CastleRuins,
            6 => Map::NorthMundeman,
            7 => Map::SouthMundeman,
            8 => Map::VerdantCoast,
            9 => Map::OtherworldArena,
            10 => Map::CastleGrounds,
            11 => Map::Sanctuary,
            12 => Map::TheTunnels,
            13 => Map::Glitch,
            14 => Map::Luddershore,
            15 => Map::TheTundra,
            16 => Map::FrozenShore,
            17 => Map::HallowGround,
            18 => Map::SouthernSwamp,
            19 => Map::DragonsLair,
            20 => Map::CorruptedCastle,
            21 => Map::CastleMonillud,
            22 => Map::ThroneRoomConfrontation,
            23 => Map::TheUnderworld,
            24 => Map::Otherworld,
            26 => Map::MoltenCavern,
            27 => Map::TheDungeons,
            28 => Map::ItemShop,
            29 => Map::Convergence,
            30 => Map::TrialOfReality,
            35 => Map::HauntedManse,
            40 => Map::SmugglersRoad,
            41 => Map::SmugglersRuin,
            _ => Map::Unknown,
        }
    }
}

impl From<Map> for u8 {
    fn from(map: Map) -> u8 {
        match map {
            Map::DustShelf => 1,
            Map::ThroneRoom => 2,
            Map::ExplodingThroneRoom => 4,
            Map::CastleRuins => 5,
            Map::NorthMundeman => 6,
            Map::SouthMundeman => 7,
            Map::VerdantCoast => 8,
            Map::OtherworldArena => 9,
            Map::CastleGrounds => 10,
            Map::Sanctuary => 11,
            Map::TheTunnels => 12,
            Map::Glitch => 13,
            Map::Luddershore => 14,
            Map::TheTundra => 15,
            Map::FrozenShore => 16,
            Map::HallowGround => 17,
            Map::SouthernSwamp => 18,
            Map::DragonsLair => 19,
            Map::CorruptedCastle => 20,
            Map::CastleMonillud => 21,
            Map::ThroneRoomConfrontation => 22,
            Map::TheUnderworld => 23,
            Map::Otherworld => 24,
            Map::MoltenCavern => 26,
            Map::TheDungeons => 27,
            Map::ItemShop => 28,
            Map::Convergence => 29,
            Map::TrialOfReality => 30,
            Map::HauntedManse => 35,
            Map::SmugglersRoad => 40,
            Map::SmugglersRuin => 41,
            Map::Unknown => u8::MAX,
        }
    }
}
