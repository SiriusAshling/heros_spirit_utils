use serde::{Serialize, Deserialize};

use crate::{inventory::Inventory, map::Map};

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

#[derive(Serialize, Deserialize)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
    Unknown,
}

impl From<u8> for Direction {
    fn from(id: u8) -> Self {
        match id {
            2 => Direction::Down,
            4 => Direction::Left,
            6 => Direction::Right,
            8 => Direction::Up,
            _ => Direction::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub map: Map,
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
