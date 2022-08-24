use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs;
use std::error::Error;

use image::ImageFormat;
use image::RgbaImage;

use crate::map::{Map, self};
use crate::util;
use crate::draw;
use crate::graphics::TileData;
use crate::zip::NamedFile;

pub fn export_tilesets(path: impl AsRef<Path>, tile_data: &TileData) -> Result<(), Box<dyn Error>> {
    let mut path = path.as_ref().to_owned();
    util::ensure_dir(&path)?;

    let mut tile8_path = path.clone();
    tile8_path.push("tile8");
    util::ensure_dir(&tile8_path)?;
    tile8_path.push("all.bmp");

    draw::draw_tile8s(tile8_path, &tile_data.tile8_list)?;

    path.push("tile16");
    util::ensure_dir(&path)?;

    draw::draw_tile16s(&path, tile_data)
}

pub fn export_files(path: impl AsRef<Path>, files: &[NamedFile], extension: impl AsRef<OsStr>) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();
    util::ensure_dir(&path)?;

    for (name, bytes) in files {
        let mut file_path = path.to_owned();
        file_path.push(name);
        file_path.set_extension(&extension);
        fs::write(file_path, bytes)?;
    }

    Ok(())
}

pub fn export_maps(path: impl AsRef<Path>, maps: &[Map], tile_data: &TileData) -> Result<(), Box<dyn Error>> {
    for map in maps {
        let mut path = path.as_ref().to_owned();
        util::ensure_dir(&path)?;
        path.push(format!("map{:02}.tmx", map.identifier));

        fs::write(path, map.to_tmx(tile_data)?)?;
    }

    Ok(())
}

pub fn export_map_image(path: impl AsRef<Path>, identifier: u8, map: &RgbaImage) -> Result<(), Box<dyn Error>> {
    let mut path = path.as_ref().to_owned();
    util::ensure_dir(&path)?;
    path.push(format!("{}_{}.png", identifier as u8, map::map_name(identifier)));

    map.save_with_format(&path, ImageFormat::Png)?;

    Ok(())
}

pub fn export_full_map_image(map: &RgbaImage) -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from("rom_files/maps/images");
    util::ensure_dir(&path)?;
    path.push("FullMap.png");

    map.save_with_format(&path, ImageFormat::Png)?;

    Ok(())
}
