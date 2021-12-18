use std::{error::Error, path::Path};

use image::{RgbaImage, ImageFormat, ImageBuffer};

use crate::{tile::{Tile8, self, Tile8Data, Tile16}, palette::{DEFAULT_PALETTE, self}, data::TERRAIN_FLAGS, sprite::Sprite, map::Map};

pub fn draw_tile8s<P: AsRef<Path>>(path: P, tile8_list: &[Tile8Data]) -> Result<(), Box<dyn Error>> {
    let len = tile8_list.len() as u32;
    let width = 80;
    let height = (len + 9) / 10 * 8;
    let mut image: RgbaImage = ImageBuffer::new(width, height);

    for index in 0..len {
        let xoffset = index % 10 * 8;
        let yoffset = index / 10 * 8;

        let tile8 = Tile8 { index: index as u16, ..Tile8::default() };

        tile::draw_tile8(tile8_list, &tile8, DEFAULT_PALETTE, &mut image, xoffset, yoffset, false);
    }

    image.save_with_format(&path, ImageFormat::Png)?;

    Ok(())
}

pub fn draw_tile16s<P: AsRef<Path>>(path: P, tile8_list: &[Tile8Data], map_tile16_list: &[Tile16], sprite_tile16_list: &[Tile16]) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref().to_owned();

    for (index, tile16) in map_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        tile::draw_tile16(tile8_list, tile16, DEFAULT_PALETTE, &mut image, 0, 0, false);

        let mut tile16_path = path.clone();
        tile16_path.push(format!("map_{}.png", index + 1));
        image.save_with_format(&tile16_path, ImageFormat::Png)?;
    }
    for (index, tile16) in sprite_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        let palette = palette::get_sprite_palette(index);
        tile::draw_tile16(tile8_list, tile16, palette, &mut image, 0, 0, false);

        let mut tile16_path = path.clone();
        tile16_path.push(format!("sprite_{}.png", index));
        image.save_with_format(&tile16_path, ImageFormat::Png)?;
    }

    Ok(())
}

pub fn draw_map<P: AsRef<Path>>(path: P, map: Map, width: u8, height: u8, tiles: Vec<Vec<u8>>, sprites: Vec<Vec<Option<Sprite>>>, tile8_list: &[Tile8Data], map_tile16_list: &[Tile16], sprite_tile16_list: &[Tile16]) -> Result<(), Box<dyn Error>> {
    let mut image: RgbaImage = ImageBuffer::new(width as u32 * 16, height as u32 * 16);

    let is_glitch = matches!(map, Map::Glitch);
    for (y, row) in tiles.into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            let mut tile = tile as usize;

            let tile_flags = if is_glitch {
                tile += 67;
                TERRAIN_FLAGS[tile - 64]
            } else {
                TERRAIN_FLAGS[tile]
            };

            if let Some(tile16) = map_tile16_list.get(tile - 1) {
                let pixel_x = x as u32 * 16;
                let pixel_y = y as u32 * 16;

                let passage = tile_flags & 0b00010010 == 0b00010010;
                let palette = palette::get_map_palette(tile, map);

                tile::draw_tile16(tile8_list, tile16, palette, &mut image, pixel_x, pixel_y, false);

                if passage {
                    tile::draw_tile8(tile8_list, &Tile8::from(825), DEFAULT_PALETTE, &mut image, pixel_x + 4, pixel_y + 4, false);
                }

                if let Some(sprite) = &sprites[y][x] {
                    match sprite {
                        Sprite::WindRoute => tile::draw_tile16(tile8_list, &sprite_tile16_list[67], palette::get_sprite_palette(67), &mut image, pixel_x, pixel_y, true),
                    }
                }
            }
        }
    }

    image.save_with_format(&path, ImageFormat::Png)?;

    Ok(())
}
