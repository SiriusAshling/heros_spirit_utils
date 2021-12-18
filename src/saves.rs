use std::error::Error;

use std::fmt::Display;
use std::path::Path;
use std::{fs, cmp::min};

use crate::map::Map;
use crate::savedata::{Position, Direction};

use crate::inventory::Inventory;
use crate::savedata::{SavePretty, SaveDat};
use crate::data::DEOBF;

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

fn unscramble(mut data: String) -> Result<SaveDat, Box<dyn Error>> {
    let rest = data.split_off(10);
    let total_steps = data.parse::<usize>()?;
    let mut len = rest.len();

    let mut first_iteration = rest[..4].to_owned();
    for index in 0..=(len - 4) / 8 {
        let begin = min(len - 1, index * 8 + 4);
        let end = min(len, begin + 8);
        let mut piece = rest[begin..end].to_owned();
        unscramble_piece(&mut piece, total_steps + index);
        first_iteration += &piece;
    }
    len = first_iteration.len();
    let mut second_iteration = String::new();
    for index in 0..=len / 8 {
        let begin = index * 8;
        let end = min(len, begin + 8);
        let mut piece = first_iteration[begin..end].to_owned();
        unscramble_piece(&mut piece, total_steps + index);
        second_iteration += &piece;
    }
    let mut second_iteration = &second_iteration[..];
    if let Some(index) = second_iteration.rfind('}') {
        second_iteration = &second_iteration[..=index];
    }

    Ok(serde_json::from_str(second_iteration)?)
}

pub fn decode<P: AsRef<Path> + Display>(path: P) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(&path)?;
    let savedat = unscramble(data)?;

    // position
    let mut position_parts = savedat.position.split('.');
    let map = Map::from(position_parts.next().ok_or("malformed data")?.parse::<u8>()?);
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
    let mut values = base64::decode(savedat.values.as_bytes())?;
    values = values.into_iter().map(|value| DEOBF[value as usize]).collect();
    let inventory = Inventory::from(values);

    let pretty = SavePretty {
        position,
        inventory,
        hearts: savedat.hearts,
        flags: savedat.flags,
        playtime: savedat.playtime,
        deaths: savedat.deaths,
        label: savedat.label,
    };

    let out = serde_json::to_string_pretty(&pretty)?;

    let mut path = path.to_string();
    path += ".decoded.json";
    fs::write(&path, out)?;

    Ok(())
}
