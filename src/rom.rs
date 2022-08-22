use std::error::Error;

use crate::error::SimpleError;
use crate::map::{self, Map};
use crate::graphics::{self, TileData};
use crate::zip::NamedFile;

pub struct Rom {
    pub tile_data: TileData,
    pub images: Vec<NamedFile>,
    pub maps: Vec<Map>,
    pub sounds: Vec<NamedFile>,
    pub music: Vec<NamedFile>,
    pub shaders: Vec<NamedFile>,
}

pub fn decode(files: Vec<(String, Vec<u8>)>) -> Result<Rom, Box<dyn Error>> {
    let mut tile8_list = None;
    let mut images = Vec::with_capacity(3);
    let mut maps = Vec::with_capacity(41);
    let mut sounds = Vec::with_capacity(39);
    let mut music = Vec::with_capacity(51);
    let mut shaders = Vec::with_capacity(1);

    for (filename, bytes) in files {
        match &filename[..] {
            "graphics" => tile8_list = Some(graphics::decode_graphics(bytes)),
            f if f.starts_with("map") => maps.push(map::decode_map(bytes)?),
            "meow" | "rawr" | "winter" => images.push((filename, bytes)),
            "retrofx" => shaders.push((filename, bytes)),
            f if f.starts_with("sfx") => sounds.push((filename, bytes)),
            f if f.starts_with("track") => music.push((filename, bytes)),
            _ => eprintln!("Unknown file in rom: {}", filename),
        }
    }

    let tile8_list = tile8_list.ok_or(SimpleError("Failed to find graphics file in ROM"))?;
    let tile_data = TileData::from(tile8_list);

    Ok(Rom { tile_data, images, maps, sounds, music, shaders })
}
