use std::{path::Path, error::Error, fs};

use crate::{map::Map, tile::{Tile16, self, Tile8Data}, sprite::Sprite, util, draw};

fn decode_graphics<P: AsRef<Path>>(path: P) -> Result<Vec<Tile8Data>, Box<dyn Error>> {
    let data = fs::read(&path)?;

    let mut bits = Vec::new();
    for byte in data {
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

    let mut path = path.as_ref()
        .parent().unwrap()
        .parent().unwrap()
        .to_owned();
    path.push("graphics");
    util::ensure_dir(&path)?;

    let mut tile8_path = path.clone();
    tile8_path.push("tile8");
    util::ensure_dir(&tile8_path)?;
    tile8_path.push("all.png");

    draw::draw_tile8s(tile8_path, &tile8_list)?;

    path.push("tile16");
    util::ensure_dir(&path)?;

    Ok(tile8_list)
}

fn decode_map<P: AsRef<Path>>(path: P, tile8_list: &[Tile8Data], map_tile16_list: &[Tile16], sprite_tile16_list: &[Tile16], enemy_tile16_list: &[Tile16]) -> Result<(), Box<dyn Error>> {
    let data = fs::read(&path)?;
    let map_id = data[0];
    let map = Map::from(map_id);
    let width = data[1];
    let width_usize = width as usize;
    let height = data[2];
    let height_usize = height as usize;

    let tiles_end = 3 + width_usize * height_usize * 7 / 8;
    let tile_bytes = &data[3..tiles_end];
    let sprite_data = &data[tiles_end..];

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

    let mut path = path.as_ref()
        .parent().unwrap()
        .parent().unwrap()
        .to_owned();
    path.push("maps");
    util::ensure_dir(&path)?;
    path.push(format!("{}_{:?}.png", map_id, map));

    draw::draw_map(&path, map, width, height, tiles, sprites, tile8_list, map_tile16_list, sprite_tile16_list, enemy_tile16_list)
}

pub fn decode<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let rom = fs::read_dir(&path)?;

    let map_tile16_list = tile::map_tile16_list();
    let sprite_tile16_list = tile::sprite_tile16_list();
    let enemy_tile16_list = tile::enemy_tile16_list();
    let mut tile8_list = None;
    let mut maps = Vec::with_capacity(30);

    for file in rom {
        let file = file?;
        let filename = file.file_name();
        let filename = filename.to_string_lossy();
        if filename == "graphics" {
            tile8_list = Some(decode_graphics(file.path())?);
        } else if filename.starts_with("map") {
            maps.push(file.path());
        }
    }

    if let Some(tile8_list) = tile8_list {
        let mut path = path.as_ref()
            .parent().unwrap()
            .to_owned();
        path.push("graphics/tile16");
        util::ensure_dir(&path)?;

        draw::draw_tile16s(&path, &tile8_list, &map_tile16_list, &sprite_tile16_list, &enemy_tile16_list)?;

        for map in maps {
            util::feedback(format!("Decode map {}", map.display()), decode_map(map, &tile8_list, &map_tile16_list, &sprite_tile16_list, &enemy_tile16_list));
        }
    }

    Ok(())
}
