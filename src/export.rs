use std::{error::Error, path::Path};

use crate::{util, draw, tile::TileData, map::Map};

pub fn export_tilesets<P: AsRef<Path>>(path: P, tile_data: &TileData) -> Result<(), Box<dyn Error>> {
    let mut path = path.as_ref().to_owned();
    util::ensure_dir(&path)?;

    let mut tile8_path = path.clone();
    tile8_path.push("tile8");
    util::ensure_dir(&tile8_path)?;
    tile8_path.push("all.png");

    draw::draw_tile8s(tile8_path, &tile_data.tile8_list)?;

    path.push("tile16");
    util::ensure_dir(&path)?;

    draw::draw_tile16s(&path, &tile_data)
}

pub fn export_map<P: AsRef<Path>>(path: P, map: Map, tile_data: &TileData) -> Result<(), Box<dyn Error>> {
    let mut path = path.as_ref().to_owned();
    util::ensure_dir(&path)?;
    path.push(format!("{}_{:?}.png", map.identifier as u8, map.identifier));

    draw::draw_map(&path, map, &tile_data)
}
