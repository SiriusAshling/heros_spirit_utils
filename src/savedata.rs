use num_enum::FromPrimitive;
use serde::{Serialize, Deserialize};

use crate::{inventory::Inventory, map::MapIdentifier};

#[derive(Serialize, Deserialize)]
pub struct SaveDat {
    pub position: String,
    pub values: String,
    pub hearts: Vec<String>,
    pub flags: serde_json::Value,
    pub playtime: usize,
    pub deaths: usize,
    pub label: String,
}

#[derive(Serialize, Deserialize, FromPrimitive)]
#[repr(u8)]
pub enum Direction {
    Down = 2,
    Left = 4,
    Right = 6,
    Up = 8,
    #[num_enum(default)]
    Unknown,
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub map: MapIdentifier,
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

#[derive(Serialize, Deserialize)]
pub struct SavePretty {
    pub position: Position,
    pub inventory: Inventory,
    pub hearts: Vec<String>,
    pub flags: serde_json::Value,
    pub playtime: usize,
    pub deaths: usize,
    pub label: String,
}
