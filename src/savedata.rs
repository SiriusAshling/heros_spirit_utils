use std::collections::HashMap;

use std::cmp::min;
use std::path::Path;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::FromRepr;

use crate::data::{DEOBF, OBF};
use crate::inventory::Inventory;
use crate::util;
use crate::Result;

#[derive(Serialize, Deserialize)]
pub struct SaveDat {
    pub position: String,
    pub values: String,
    pub hearts: Vec<String>,
    pub flags: HashMap<String, bool>,
    pub playtime: usize,
    pub deaths: usize,
    pub kills: usize,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, FromRepr)]
#[repr(u8)]
pub enum Direction {
    Down = 2,
    Left = 4,
    Right = 6,
    Up = 8,
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub map: u8,
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}
impl Position {
    pub fn encode(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.map, self.x, self.y, self.direction as u8
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct SavePretty {
    pub steps: usize,
    pub position: Position,
    pub inventory: Inventory,
    pub hearts: Vec<String>,
    pub flags: HashMap<String, bool>,
    pub playtime: usize,
    pub deaths: usize,
    pub kills: usize,
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

fn unscramble(mut data: String) -> Result<(usize, SaveDat)> {
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

fn scramble(steps: usize, data: &SaveDat) -> Result<String> {
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

    // I wish I knew why the output is broken sometimes...
    if let Some((index, _)) = second_iteration.match_indices('}').nth(2) {
        second_iteration.remove(index);
    }

    Ok(second_iteration)
}

pub fn decode(path: &str, write_completion: bool) -> Result<SavePretty> {
    let data = util::read_to_string(path)?;
    let (steps, savedat) = unscramble(data)?;
    let SaveDat {
        position,
        values,
        hearts,
        flags,
        playtime,
        deaths,
        kills,
        label,
    } = savedat;

    // position
    let mut position_parts = position.split('.');
    let map = position_parts
        .next()
        .ok_or("malformed data")?
        .parse::<u8>()?;
    let x = position_parts.next().ok_or("malformed data")?.parse()?;
    let y = position_parts.next().ok_or("malformed data")?.parse()?;
    let direction = Direction::from_repr(
        position_parts
            .next()
            .ok_or("malformed data")?
            .parse::<u8>()?,
    )
    .ok_or("unknown direction")?;
    let position = Position {
        map,
        x,
        y,
        direction,
    };

    // values
    let mut values = BASE64_STANDARD.decode(values)?;
    values = values
        .into_iter()
        .map(|value| DEOBF[value as usize])
        .collect();
    let inventory = Inventory::from(values);

    let pretty = SavePretty {
        steps,
        position,
        inventory,
        hearts,
        flags,
        playtime,
        deaths,
        kills,
        label,
    };

    let out = serde_json::to_string_pretty(&pretty)?;
    util::write(format!("{path}.json"), out)?;

    if write_completion {
        let completion_column = pretty.inventory.completion_column().into_iter().join("\n");
        util::write(
            format!("completion/{path}_completion.txt"),
            completion_column,
        )?;
    }

    Ok(pretty)
}

pub fn encode(path: impl AsRef<Path>) -> Result<()> {
    let read_path = path.as_ref().with_extension("json");
    let data = util::read_to_string(&read_path)?;

    let SavePretty {
        steps,
        position,
        inventory,
        hearts,
        flags,
        playtime,
        deaths,
        kills,
        label,
    } = serde_json::from_str(&data)?;
    let values = inventory
        .into_vec()
        .into_iter()
        .map(|value| OBF[value as usize])
        .collect::<Vec<_>>();
    let values = BASE64_STANDARD.encode(values);
    let position = position.encode();

    let savedat = SaveDat {
        position,
        values,
        hearts,
        flags,
        playtime,
        deaths,
        kills,
        label,
    };

    let out = scramble(steps, &savedat)?;
    util::write(path, out)?;

    Ok(())
}
