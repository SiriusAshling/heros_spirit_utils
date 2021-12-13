mod saveformat;
mod map;
mod inventory;
mod data;

use std::{fs, cmp::min};

use map::Map;
use saveformat::{Position, Direction};

use crate::inventory::Inventory;
use crate::saveformat::{SavePretty, SaveDat};
use crate::data::DEOBF;

fn unshift(s: &mut String, offset: usize) {
    let result = s.split_off(s.len() - offset);
    *s = result + &s;
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

fn unscramble(path: &str) {
    let mut data = fs::read_to_string(path).unwrap();
    let rest = data.split_off(10);
    let total_steps = data.parse::<usize>().unwrap();
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
    let remove_trail = &second_iteration[..=second_iteration.rfind("}").unwrap()];

    let savedat: SaveDat = serde_json::from_str(&remove_trail).unwrap();

    // position
    let mut position_parts = savedat.position.split(".");
    let map = Map::from(position_parts.next().unwrap().parse::<u8>().unwrap());
    let x = position_parts.next().unwrap().parse().unwrap();
    let y = position_parts.next().unwrap().parse().unwrap();
    let direction = Direction::from(position_parts.next().unwrap().parse::<u8>().unwrap());
    let position = Position {
        map,
        x,
        y,
        direction,
    };

    // values
    let mut values = base64::decode(savedat.values.as_bytes()).unwrap();
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

    let out = serde_json::to_string_pretty(&pretty).unwrap();

    let mut path = path.to_owned();
    path += ".decoded.json";
    fs::write(&path, out).unwrap();
}

fn main() {
    unscramble("savedata");
    unscramble("savedatb");
    unscramble("savedatc");
}
