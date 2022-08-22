use std::error::Error;

use crate::error::SimpleError;
use crate::map::{Map, MapIdentifier};
use crate::tile::{TileData, Tile8Data};
use crate::sprite::SpriteData;
use crate::zip::NamedFile;

pub struct Rom {
    pub tile_data: TileData,
    pub images: Vec<NamedFile>,
    pub maps: Vec<Map>,
    pub sounds: Vec<NamedFile>,
    pub music: Vec<NamedFile>,
    pub shaders: Vec<NamedFile>,
}

fn decode_graphics(bytes: Vec<u8>) -> Vec<Tile8Data> {
    bytes.chunks(2).collect::<Vec<_>>().chunks(8).map(|tile8|
        tile8.iter().map(|col|
            (0..8).map(|index|
                (col[0] & 1 << (7 - index) != 0) as u8 +
                (col[1] & 1 << (7 - index) != 0) as u8 * 2
            ).collect()
        ).collect()
    ).collect()
}

pub fn encode_graphics(tile8_list: Vec<Tile8Data>) -> Vec<u8> {
    tile8_list.into_iter().flat_map(|tile8|
        tile8.into_iter().flat_map(|col|
            [
                (0..8).fold(0, |acc, index| acc | ((col[index] & 1) << (7 - index))),
                (0..8).fold(0, |acc, index| acc | ((col[index] & 2) >> 1 << (7 - index)))
            ]
        )
    ).collect()
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

    let mut tile_bits = Vec::with_capacity(tile_bytes.len() * 8);

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
        let (x, y, sprite) = SpriteData::read(sprite_data, &mut sprite_index)?;
        sprites[y as usize][x as usize] = Some(sprite);
    }

    let identifier = MapIdentifier::from(map_id);

    Ok(Map { identifier, tiles, sprites })
}

pub fn encode_map(map: Map) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(0);

    bytes.push(map.identifier as u8);
    let width = map.tiles[0].len();
    let height = map.tiles.len();
    bytes.push(width as u8);
    bytes.push(height as u8);

    bytes.extend(
        map.tiles.into_iter().flat_map(|row|
            row.into_iter().flat_map(|tile|
                (0..7).map(move |index| tile & (1 << (6 - index)) != 0)
            )
        ).collect::<Vec<_>>().chunks(8).enumerate().map(|(index, bits)| {
            let mut byte = 0;
            let mut bits = bits.to_vec();
            while bits.len() < 8 {
                bits.push(false);
            }

            for bit_index in (0..8).rev() {
                let position = (10963 * map.identifier as usize + index * 8) % (1 + bit_index);
                byte |= (bits.remove(position) as u8) << bit_index;
            }
            byte
        })
    );

    bytes.extend(
        map.sprites.into_iter().enumerate().flat_map(|(y, row)|
            row.into_iter().enumerate().filter_map(move |(x, sprite)| sprite.map(|sprite| (x, y, sprite)))
        ).flat_map(|(x, y, sprite)| {
            let mut bytes = vec![sprite.kind, x as u8, y as u8];
            bytes.extend(sprite.extra_bytes);
            bytes
        })
    );

    bytes
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
            "graphics" => tile8_list = Some(decode_graphics(bytes)),
            f if f.starts_with("map") => maps.push(decode_map(bytes)?),
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
