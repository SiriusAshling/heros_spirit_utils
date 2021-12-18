use num_enum::FromPrimitive;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum Map {
    DustShelf = 1,
    ThroneRoom,
    ExplodingThroneRoom = 4,
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
    MoltenCavern = 26,
    TheDungeons,
    ItemShop,
    Convergence,
    TrialOfReality,
    HauntedManse = 35,
    SmugglersRoad = 40,
    SmugglersRuin,
    #[num_enum(default)]
    Unknown,
}
