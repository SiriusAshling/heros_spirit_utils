use std::{error::Error, path::Path};

use image::{RgbaImage, ImageFormat, ImageBuffer};

use crate::{tile::{Tile8, self, Tile8Data, Tile16}, palette::{DEFAULT_PALETTE, self}, data::{TERRAIN_FLAGS, BRIGHT_MAPS}, sprite::{Sprite, Collectible, Enemy}, map::Map};

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

pub fn draw_tile16s<P: AsRef<Path>>(path: P, tile8_list: &[Tile8Data], map_tile16_list: &[Tile16], sprite_tile16_list: &[Tile16], enemy_tile16_list: &[Tile16]) -> Result<(), Box<dyn Error>> {
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
    for (index, tile16) in enemy_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        let palette = palette::get_enemy_palette(index);
        tile::draw_tile16(tile8_list, tile16, palette, &mut image, 0, 0, false);

        let mut tile16_path = path.clone();
        tile16_path.push(format!("enemy_{}.png", index));
        image.save_with_format(&tile16_path, ImageFormat::Png)?;
    }

    Ok(())
}

pub fn draw_map<P: AsRef<Path>>(path: P, map: Map, width: u8, height: u8, tiles: Vec<Vec<u8>>, sprites: Vec<Vec<Option<Sprite>>>, tile8_list: &[Tile8Data], map_tile16_list: &[Tile16], sprite_tile16_list: &[Tile16], enemy_tile16_list: &[Tile16]) -> Result<(), Box<dyn Error>> {
    let mut image: RgbaImage = ImageBuffer::new(width as u32 * 16, height as u32 * 16);

    let is_glitch = matches!(map, Map::Glitch);
    let bright_map = BRIGHT_MAPS.contains(&(map as u8));

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
            }
        }
    }

    for (y, row) in sprites.into_iter().enumerate() {
        for (x, sprite) in row.into_iter().enumerate() {
            if let Some(sprite) = sprite {
                let pixel_x = x as u32 * 16;
                let pixel_y = y as u32 * 16;

                if let Sprite::Enemy(enemy) = sprite {
                    let sprite_index = match enemy {
                        Enemy::Dragon => {
                            let palette = match map {
                                Map::CastleMonillud => palette::lookup_palette([11, 9, 25, 64]),
                                Map::TheUnderworld => palette::lookup_palette([7, 22, 23, 64]),
                                _ => palette::lookup_palette([12, 18, 16, 64]),
                            };

                            let mut index = 561;
                            for tile8_y in 0..4 {
                                for tile8_x in 0..6 {
                                    if tile8_y == 0 && (tile8_x == 0 || tile8_x > 3) { continue }
                                    let tile8 = Tile8::from(index);
                                    let xoffset = pixel_x + tile8_x * 8;
                                    let yoffset = pixel_y + tile8_y * 8;
                                    tile::draw_tile8(tile8_list, &tile8, palette, &mut image, xoffset, yoffset, true);
                                    index += 1;
                                }
                            }

                            continue;
                        },
                        Enemy::Basilisk => {
                            let palette = palette::lookup_palette([7, 22, 23, 64]);
                            let mut index = 582;
                            for tile8_y in 0..4 {
                                for tile8_x in 0..2 {
                                    let tile8 = Tile8::from(index);
                                    let xoffset = pixel_x + tile8_x * 8;
                                    let yoffset = pixel_y + tile8_y * 8;
                                    tile::draw_tile8(tile8_list, &tile8, palette, &mut image, xoffset, yoffset, true);
                                    index += 1;
                                }
                            }

                            continue;
                        },
                        Enemy::CloneP => 30,
                        Enemy::CloneW => 31,
                        Enemy::Witch => 34,
                        // Enemy::Ragnarok => 20,
                        // Enemy::EvilBunny => 42,
                        // Enemy::DarkGhost => 43,
                        _ => enemy as usize,
                    };

                    let tile16 = &enemy_tile16_list[sprite_index];
                    let palette = palette::get_enemy_palette(sprite_index);
                    tile::draw_tile16(tile8_list, tile16, palette, &mut image, pixel_x, pixel_y, true);
                    continue;
                }

                let sprite_index = match sprite {
                    Sprite::Collectible(collectible) => match collectible {
                        Collectible::Sword | Collectible::SilverKey if bright_map => collectible as usize + 18,
                        Collectible::PossumCoin => 100,
                        _ => collectible as usize + 17,
                    },
                    Sprite::Door(door) => door as usize + 2,
                    Sprite::WindRoute => 67,
                    Sprite::Save => 0,
                    _ => usize::MAX,
                };
                if sprite_index == usize::MAX { continue }

                let tile16 = &sprite_tile16_list[sprite_index];
                let palette = palette::get_sprite_palette(sprite_index);
                tile::draw_tile16(tile8_list, tile16, palette, &mut image, pixel_x, pixel_y, true);
            }
        }
    }
    image.save_with_format(&path, ImageFormat::Png)?;

    Ok(())
}
