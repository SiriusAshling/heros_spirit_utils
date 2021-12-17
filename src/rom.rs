use std::{path::Path, error::Error, fs};

use image::{ImageBuffer, ImageFormat, RgbaImage};

use crate::{map::Map, data::TERRAIN_FLAGS, tile::{Tile16, self, Tile8Data, Tile8}, palette::{self, DEFAULT_PALETTE}};

fn decode_graphics<P: AsRef<Path>>(path: P, tile16_list: &[Tile16]) -> Result<Vec<Tile8Data>, Box<dyn Error>> {
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
        for x in 0..8 {
            for y in 0..8 {
                let mut color = 0u8;
                let bit_index = x * 16 + y;
                if texture_bits[bit_index] {
                    color += 1;
                }
                if texture_bits[bit_index + 8] {
                    color += 2;
                }
                pixels[x][y] = color;
            }
        }

        tile8_list.push(pixels);
    }

    fs::create_dir("graphics");

    for (index, tile16) in tile16_list.into_iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        tile::draw_tile16(&tile8_list, tile16, DEFAULT_PALETTE, &mut image, 0, 0, false);

        let mut path = path.as_ref()
            .parent().unwrap()
            .parent().unwrap()
            .to_owned();
        fs::create_dir("graphics/tile16");
        path.push("graphics/tile16");
        path.push(format!("{}.png", index + 1));
        image.save_with_format(&path, ImageFormat::Png)?;
    }

    let len = tile8_list.len() as u32;
    let width = 80;
    let height = len + 9 / 10;
    let mut image: RgbaImage = ImageBuffer::new(width, height);

    for index in 0..len {
        let xoffset = index % 10 * 8;
        let yoffset = index / 10 * 8;

        let tile8 = Tile8 { index: index as u16, ..Tile8::default() };

        tile::draw_tile8(&tile8_list, &tile8, DEFAULT_PALETTE, &mut image, xoffset, yoffset, false);
    }
    let mut path = path.as_ref()
        .parent().unwrap()
        .parent().unwrap()
        .to_owned();
    fs::create_dir("graphics/tile8");
    path.push("graphics/tile8/all.png");
    image.save_with_format(&path, ImageFormat::Png)?;

    Ok(tile8_list)
}

fn decode_map<P: AsRef<Path>>(path: P, tile8_list: &[Tile8Data], tile16_list: &[Tile16]) -> Result<(), Box<dyn Error>> {
    let data = fs::read(&path)?;
    let map_id = data[0];
    let map = Map::from(map_id);
    let width = data[1] as usize;
    let height = data[2] as usize;

    let tiles_end = 3 + width * height * 7 / 8;
    let tile_bytes = &data[3..tiles_end];
    let objects = &data[tiles_end..];

    let tile_bytes_len = tile_bytes.len();
    let mut tile_bits = Vec::with_capacity(tile_bytes_len * 8);

    for (index, byte) in tile_bytes.into_iter().enumerate() {
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
            for bit_index in 0..7 {
                let bit = tile_bits[bit_index];
                if bit {
                    tile |= 1 << (6 - bit_index);
                }
            }
            Some(tile)
        })
    };

    let mut tiles = Vec::with_capacity(height);
    for _ in 0..height {
        let mut row = Vec::with_capacity(width);

        for _ in 0..width {
            if let Some(tile) = read_tile() {
                row.push(tile);
            }
        }

        tiles.push(row);
    }

    let display = tiles.iter().map(|row| format!("{:?}", row)).collect::<Vec<_>>().join("\n");

    let mut path = path.as_ref()
        .parent().unwrap()
        .parent().unwrap()
        .to_owned();
    fs::create_dir("maps");
    path.push("maps");
    path.push(format!("{:?}", Map::from(map_id)));
    fs::write(&path, format!("{}", display))?;

    let mut image: RgbaImage = ImageBuffer::new(width as u32 * 16, height as u32 * 16);

    let is_glitch = map_id == 13;
    for (y, row) in tiles.into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            let mut tile = tile as usize;
            if let Some(tile16) = tile16_list.get(tile - 1) {
                let x = x as u32 * 16;
                let y = y as u32 * 16;

                let tile_flags = if is_glitch {
                    tile += 67;
                    TERRAIN_FLAGS[tile - 64]
                } else {
                    TERRAIN_FLAGS[tile]
                };
                let passage = tile_flags & 0b00010010 == 0b00010010;
                let palette = palette::get_palette(tile, map);

                tile::draw_tile16(&tile8_list, tile16, palette, &mut image, x, y, false);

                if passage {
                    tile::draw_tile8(&tile8_list, &Tile8::from(825), DEFAULT_PALETTE, &mut image, x + 4, y + 4, false);
                }
            }
        }
    }

    path.set_extension("png");
    image.save_with_format(&path, ImageFormat::Png)?;

    Ok(())
}

pub fn decode<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let rom = fs::read_dir(path)?;

    let tile16_list = tile::tile16_list();
    let mut tile8_list = None;
    let mut maps = Vec::with_capacity(30);

    for file in rom {
        let file = file?;
        let filename = file.file_name();
        let filename = filename.to_string_lossy();
        if filename == "graphics" {
            tile8_list = Some(decode_graphics(file.path(), &tile16_list)?);
        } else if filename.starts_with("map") {
            maps.push(file.path());
        }
    }

    if let Some(tile8_list) = tile8_list {
        for map in maps {
            decode_map(map, &tile8_list, &tile16_list)?;
        }
    }

    Ok(())
}
