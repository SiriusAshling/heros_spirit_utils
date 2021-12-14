use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Map {
    DustShelf,
    ThroneRoom,
    ExplodingThroneRoom,
    CastleRuins,
    NorthMundeman,
    SouthMundeman,
    VerdantCoast,
    Otherworld,
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
    TheUnderworld,
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
            9 => Map::Otherworld,
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
            22 => Map::ThroneRoom,
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

pub enum Tile {
    
}
