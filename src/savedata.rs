use std::error::Error;
use std::path::Path;
use std::{fs, cmp::min};

use num_enum::FromPrimitive;
use serde::{Serialize, Deserialize};

use crate::map::MapIdentifier;
use crate::inventory::Inventory;
use crate::data::{OBF, DEOBF};

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

#[derive(Serialize, Deserialize, Clone, Copy, FromPrimitive)]
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
impl Position {
    pub fn encode(&self) -> String {
        format!("{}.{}.{}.{}", self.map as u8, self.x, self.y, self.direction as u8)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SavePretty {
    pub steps: usize,
    pub position: Position,
    pub inventory: Inventory,
    pub hearts: Vec<String>,
    pub flags: serde_json::Value,
    pub playtime: usize,
    pub deaths: usize,
    pub label: String,
}

fn shift(s: &mut String, offset: usize) {
    let result = s.split_off(offset);
    *s = result + s;
}
fn unshift(s: &mut String, offset: usize) {
    let result = s.split_off(s.len() - offset);
    *s = result + s;
}
fn flip(s: &mut String) {
    *s = s.chars().rev().collect();
}

fn unscramble_piece(s: &mut String, seed: usize) {
    let len = s.len();

    if (seed + 1) % 3 == 0 {
        flip(s);
    }
    if seed % 3 == 0 && len == 8 {
        unshift(s, (seed + 5) % 7 + 1);
    }
    if (seed + 2) % 3 == 0 {
        flip(s);
    }
    if seed % 3 == 0 {
        flip(s);
    }
    if (seed + 2) % 3 == 0 && len == 8 {
        unshift(s, (seed + 3) % 7 + 1);
    }
    if (seed + 1) % 3 == 0 {
        flip(s);
    }
    if (seed + 2) % 3 == 0 {
        flip(s);
    }
    if (seed + 1) % 3 == 0 && len == 8 {
        unshift(s, seed % 7 + 1);
    }
    if seed % 3 == 0 {
        flip(s);
    }
}

fn scramble_piece(s: &mut String, seed: usize) {
    let len = s.len();

    if seed % 3 == 0 {
        flip(s);
    }
    if (seed + 1) % 3 == 0 && len == 8 {
        shift(s, seed % 7 + 1);
    }
    if (seed + 2) % 3 == 0 {
        flip(s);
    }
    if (seed + 1) % 3 == 0 {
        flip(s);
    }
    if (seed + 2) % 3 == 0 && len == 8 {
        shift(s, (seed + 3) % 7 + 1);
    }
    if seed % 3 == 0 {
        flip(s);
    }
    if (seed + 2) % 3 == 0 {
        flip(s);
    }
    if seed % 3 == 0 && len == 8 {
        shift(s, (seed + 5) % 7 + 1);
    }
    if (seed + 1) % 3 == 0 {
        flip(s);
    }
}

fn unscramble(mut data: String) -> Result<(usize, SaveDat), Box<dyn Error>> {
    let rest = data.split_off(10);

    let steps = data.parse()?;
    let mut len = rest.len();

    let mut first_iteration = rest[..4].to_owned();
    for index in 0..=(len - 4) / 8 {
        let begin = min(len - 1, index * 8 + 4);
        let end = min(len, begin + 8);
        let mut piece = rest[begin..end].to_owned();
        unscramble_piece(&mut piece, steps + index);
        first_iteration += &piece;
    }
    len = first_iteration.len();
    let mut second_iteration = String::new();
    for index in 0..=len / 8 {
        let begin = index * 8;
        let end = min(len, begin + 8);
        let mut piece = first_iteration[begin..end].to_owned();
        unscramble_piece(&mut piece, steps + index);
        second_iteration += &piece;
    }
    // Sometimes there are trailing symbols...
    let mut depth = 0;
    let mut end = second_iteration.len();
    for (index, char) in second_iteration.char_indices() {
        match char {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            end = index + 1;
            break;
        }
    }

    Ok((steps, serde_json::from_str(&second_iteration[..end])?))
}

fn scramble(steps: usize, data: SaveDat) -> Result<String, Box<dyn Error>> {
    let mut first_iteration = String::new();
    let data = serde_json::to_string(&data)?;
    let mut len = data.len();

    for index in 0..=len / 8 {
        let begin = index * 8;
        let end = min(len, begin + 8);
        let mut piece = data[begin..end].to_owned();
        scramble_piece(&mut piece, steps + index);
        first_iteration += &piece;
    }
    len = first_iteration.len();
    let mut second_iteration = format!("{:010}{}", steps, &first_iteration[..4]);
    for index in 0..=(len - 4) / 8 {
        let begin = min(len - 1, index * 8 + 4);
        let end = min(len, begin + 8);
        let mut piece = first_iteration[begin..end].to_owned();
        scramble_piece(&mut piece, steps + index);
        second_iteration += &piece;
    }

    Ok(second_iteration)
}

pub fn decode(path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(&path)?;
    let (steps, savedat) = unscramble(data)?;

    // position
    let mut position_parts = savedat.position.split('.');
    let map = MapIdentifier::from(position_parts.next().ok_or("malformed data")?.parse::<u8>()?);
    let x = position_parts.next().ok_or("malformed data")?.parse()?;
    let y = position_parts.next().ok_or("malformed data")?.parse()?;
    let direction = Direction::from(position_parts.next().ok_or("malformed data")?.parse::<u8>()?);
    let position = Position {
        map,
        x,
        y,
        direction,
    };

    // values
    let mut values = base64::decode(savedat.values)?;
    values = values.into_iter().map(|value| DEOBF[value as usize]).collect();
    let inventory = Inventory::from(values);

    let pretty = SavePretty {
        steps,
        position,
        inventory,
        hearts: savedat.hearts,
        flags: savedat.flags,
        playtime: savedat.playtime,
        deaths: savedat.deaths,
        label: savedat.label,
    };

    let out = serde_json::to_string_pretty(&pretty)?;

    let mut path = path.as_ref().with_extension("json");
    fs::write(&path, out)?;

    let completion_column = pretty.inventory.into_completion_column().into_iter().map(|value| value.to_string()).collect::<Vec<_>>().join("\n");
    path.set_extension("completion");
    fs::write(&path, completion_column)?;

    Ok(())
}

pub fn encode(path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let read_path = path.as_ref().with_extension("json");
    let data = fs::read_to_string(&read_path)?;

    let SavePretty { steps, position, inventory, hearts, flags, playtime, deaths, label } = serde_json::from_str(&data)?;
    let values = inventory.into_vec().into_iter().map(|value| OBF[value as usize]).collect::<Vec<_>>();
    let values = base64::encode(values);
    let position = position.encode();

    let savedat = SaveDat { position, values, hearts, flags, playtime, deaths, label };

    let out = scramble(steps, savedat)?;
    fs::write(path, out)?;

    Ok(())
}
