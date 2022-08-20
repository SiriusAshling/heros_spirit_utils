use std::error::Error;
use std::fmt::{self, Display};

use crate::map::{Map, MapIdentifier};
use crate::tile::{self, TileData, Tile8Data};
use crate::sprite::Sprite;
use crate::zip::NamedFile;

pub struct Rom {
    pub tile_data: TileData,
    pub images: Vec<NamedFile>,
    pub maps: Vec<Map>,
    pub sounds: Vec<NamedFile>,
    pub music: Vec<NamedFile>,
    pub shaders: Vec<NamedFile>,
}

#[derive(Debug)]
pub struct DecodeError {
    description: String,
}
impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
impl Error for DecodeError {}

fn decode_graphics(bytes: Vec<u8>) -> Result<Vec<Tile8Data>, Box<dyn Error>> {
    let mut bits = Vec::new();
    for byte in bytes {
        for bit_index in 0..8 {
            let bit = byte & 1 << (7 - bit_index) != 0;
            bits.push(bit);
        }
    }

    let mut tile8_list = Vec::with_capacity(848);

    for index in 0..848 {
        let texture_bits = &bits[index * 128..(index + 1) * 128];

        let mut pixels = vec![vec![0; 8]; 8];
        for (x, col) in pixels.iter_mut().enumerate() {
            for (y, pixel) in col.iter_mut().enumerate() {
                let mut color = 0u8;
                let bit_index = x * 16 + y;
                if texture_bits[bit_index] {
                    color += 1;
                }
                if texture_bits[bit_index + 8] {
                    color += 2;
                }
                *pixel = color;
            }
        }

        tile8_list.push(pixels);
    }

    Ok(tile8_list)
}

fn decode_map(bytes: Vec<u8>) -> Result<Map, Box<dyn Error>> {
    let map_id = bytes[0];
    let width = bytes[1];
    let width_usize = width as usize;
    let height = bytes[2];
    let height_usize = height as usize;

    let tiles_end = 3 + width_usize * height_usize * 7 / 8;
    let tile_bytes = &bytes[3..tiles_end];
    let sprite_data = &bytes[tiles_end..];

    let tile_bytes_len = tile_bytes.len();
    let mut tile_bits = Vec::with_capacity(tile_bytes_len * 8);

    for (index, byte) in tile_bytes.iter().enumerate() {
        let mut bits = Vec::with_capacity(8);

        for bit_index in 0..8 {
            let bit = byte & (1 << bit_index) != 0;
            let position = (10963 * map_id as usize + index * 8) % (1 + bit_index);
            bits.insert(position, bit);
        }

        tile_bits.append(&mut bits);
    }

    let mut tile_chunks = tile_bits.chunks(7);

    let mut read_tile = || {
        tile_chunks.next().and_then(|tile_bits| {
            if tile_bits.len() < 7 {
                return None;
            }

            let mut tile = 0u8;
            for (bit_index, bit) in tile_bits.iter().enumerate().take(7) {
                if *bit {
                    tile |= 1 << (6 - bit_index);
                }
            }
            Some(tile)
        })
    };

    let mut tiles = Vec::with_capacity(height_usize);
    for _ in 0..height {
        let mut row = Vec::with_capacity(width_usize);

        for _ in 0..width {
            if let Some(tile) = read_tile() {
                row.push(tile);
            }
        }

        tiles.push(row);
    }

    let mut sprites = vec![vec![None; width_usize]; height_usize];
    let mut sprite_index = 0;
    let len = sprite_data.len();
    while sprite_index < len {
        let sprite = Sprite::from(sprite_data[sprite_index]);
        let x = sprite_data[sprite_index + 1] as usize;
        let y = sprite_data[sprite_index + 2] as usize;
        sprite_index += sprite.data_bytes();
        sprites[y][x] = Some(sprite);
    }

    let identifier = MapIdentifier::from(map_id);

    Ok(Map { identifier, tiles, sprites })
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
            "graphics" => tile8_list = Some(decode_graphics(bytes)?),
            f if f.starts_with("map") => maps.push(decode_map(bytes)?),
            "meow" | "rawr" | "winter" => images.push((filename, bytes)),
            "retrofx" => shaders.push((filename, bytes)),
            f if f.starts_with("sfx") => sounds.push((filename, bytes)),
            f if f.starts_with("track") => music.push((filename, bytes)),
            _ => eprintln!("Unknown file in rom: {}", filename),
        }
    }

    let tile8_list = tile8_list.ok_or_else(|| DecodeError { description: String::from("Failed to find graphics file in ROM") })?;
    let map_tile16_list = tile::map_tile16_list();
    let sprite_tile16_list = tile::sprite_tile16_list();
    let enemy_tile16_list = tile::enemy_tile16_list();
    let tile_data = TileData { tile8_list, map_tile16_list, sprite_tile16_list, enemy_tile16_list };

    Ok(Rom { tile_data, images, maps, sounds, music, shaders })
}
