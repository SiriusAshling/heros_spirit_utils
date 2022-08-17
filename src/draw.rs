use std::{error::Error, path::Path};

use image::{RgbaImage, ImageFormat, ImageBuffer};

use crate::{tile::{Tile8, self, Tile8Data, TileData}, palette::{DEFAULT_PALETTE, self}, data::{TERRAIN_FLAGS, BRIGHT_MAPS}, sprite::{Sprite, Collectible, Enemy}, map::{MapIdentifier, Map}};

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

pub fn draw_tile16s<P: AsRef<Path>>(path: P, tile_data: &TileData) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref().to_owned();

    for (index, tile16) in tile_data.map_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        tile::draw_tile16(&tile_data.tile8_list, tile16, DEFAULT_PALETTE, &mut image, 0, 0, false);

        let mut tile16_path = path.clone();
        tile16_path.push(format!("map_{}.png", index + 1));
        image.save_with_format(&tile16_path, ImageFormat::Png)?;
    }
    for (index, tile16) in tile_data.sprite_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        let palette = palette::get_sprite_palette(index);
        tile::draw_tile16(&tile_data.tile8_list, tile16, palette, &mut image, 0, 0, false);

        let mut tile16_path = path.clone();
        tile16_path.push(format!("sprite_{}.png", index));
        image.save_with_format(&tile16_path, ImageFormat::Png)?;
    }
    for (index, tile16) in tile_data.enemy_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        let palette = palette::get_enemy_palette(index);
        tile::draw_tile16(&tile_data.tile8_list, tile16, palette, &mut image, 0, 0, false);

        let mut tile16_path = path.clone();
        tile16_path.push(format!("enemy_{}.png", index));
        image.save_with_format(&tile16_path, ImageFormat::Png)?;
    }

    Ok(())
}

pub fn draw_map(map: Map, tile_data: &TileData) -> Result<RgbaImage, Box<dyn Error>> {
    let width = map.tiles[0].len();
    let height = map.tiles.len();
    let mut image: RgbaImage = ImageBuffer::new(width as u32 * 16, height as u32 * 16);

    let is_glitch = matches!(map.identifier, MapIdentifier::Glitch);
    let bright_map = BRIGHT_MAPS.contains(&(map.identifier as u8));

    for (y, row) in map.tiles.into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            let mut tile = tile as usize;

            let tile_flags = if is_glitch {
                tile += 67;
                TERRAIN_FLAGS[tile - 64]
            } else {
                TERRAIN_FLAGS[tile]
            };

            if let Some(tile16) = tile_data.map_tile16_list.get(tile - 1) {
                let pixel_x = x as u32 * 16;
                let pixel_y = y as u32 * 16;

                let passage = tile_flags & 0b00010010 == 0b00010010;
                let palette = palette::get_map_palette(tile, map.identifier);

                tile::draw_tile16(&tile_data.tile8_list, tile16, palette, &mut image, pixel_x, pixel_y, false);

                if passage {
                    tile::draw_tile8(&tile_data.tile8_list, &Tile8::from(825), DEFAULT_PALETTE, &mut image, pixel_x + 4, pixel_y + 4, false);
                }
            }
        }
    }

    for (y, row) in map.sprites.into_iter().enumerate() {
        for (x, sprite) in row.into_iter().enumerate() {
            if let Some(sprite) = sprite {
                let pixel_x = x as u32 * 16;
                let pixel_y = y as u32 * 16;

                if let Sprite::Enemy(enemy) = sprite {
                    let sprite_index = match enemy {
                        Enemy::Dragon => {
                            let palette = match map.identifier {
                                MapIdentifier::CastleMonillud => palette::lookup_palette([11, 9, 25, 64]),
                                MapIdentifier::TheUnderworld => palette::lookup_palette([7, 22, 23, 64]),
                                _ => palette::lookup_palette([12, 18, 16, 64]),
                            };

                            let mut index = 561;
                            for tile8_y in 0..4 {
                                for tile8_x in 0..6 {
                                    if tile8_y == 0 && (tile8_x == 0 || tile8_x > 3) { continue }
                                    let tile8 = Tile8::from(index);
                                    let xoffset = pixel_x + tile8_x * 8;
                                    let yoffset = pixel_y + tile8_y * 8;
                                    tile::draw_tile8(&tile_data.tile8_list, &tile8, palette, &mut image, xoffset, yoffset, true);
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
                                    tile::draw_tile8(&tile_data.tile8_list, &tile8, palette, &mut image, xoffset, yoffset, true);
                                    index += 1;
                                }
                            }

                            continue;
                        },
                        // Enemy::Ragnarok => 20,
                        // Enemy::EvilBunny => 42,
                        // Enemy::DarkGhost => 43,
                        _ => enemy as usize,
                    };

                    let tile16 = &tile_data.enemy_tile16_list[sprite_index];
                    let palette = palette::get_enemy_palette(sprite_index);
                    tile::draw_tile16(&tile_data.tile8_list, tile16, palette, &mut image, pixel_x, pixel_y, true);
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

                let tile16 = &tile_data.sprite_tile16_list[sprite_index];
                let palette = palette::get_sprite_palette(sprite_index);
                tile::draw_tile16(&tile_data.tile8_list, tile16, palette, &mut image, pixel_x, pixel_y, true);
            }
        }
    }

    Ok(image)
}

pub fn merge_maps(maps: Vec<(MapIdentifier, RgbaImage)>) -> Result<RgbaImage, Box<dyn Error>> {
    let mut image = RgbaImage::new(7808, 9008);

    for (identifier, map) in maps {
        let (x_offset, mut y) = map_offset(identifier);

        for row in map.rows() {
            let mut x = x_offset;
            for pixel in row {
                image.put_pixel(x, y, *pixel);
                x += 1;
            }
            y += 1;
        }
    }

    Ok(image)
}

fn map_offset(identifier: MapIdentifier) -> (u32, u32) {
    match identifier {
        MapIdentifier::DustShelf => (640, 3968),
        MapIdentifier::ThroneRoom => (3840, 6976),
        MapIdentifier::ExplodingThroneRoom => (2944, 5696),
        MapIdentifier::CastleRuins => (4224, 5696),
        MapIdentifier::NorthMundeman => (3712, 1600),
        MapIdentifier::SouthMundeman => (3712, 2624),
        MapIdentifier::VerdantCoast => (3712, 3648),
        MapIdentifier::OtherworldArena => (5760, 4672),
        MapIdentifier::CastleGrounds => (1664, 3648),
        MapIdentifier::Sanctuary => (1152, 3968),
        MapIdentifier::TheTunnels => (2016, 3264),
        MapIdentifier::Glitch => (3264, 7360),
        MapIdentifier::Luddershore => (4736, 1600),
        MapIdentifier::TheTundra => (1664, 1216),
        MapIdentifier::FrozenShore => (3712, 576),
        MapIdentifier::HallowGround => (640, 1920),
        MapIdentifier::SouthernSwamp => (640, 2944),
        MapIdentifier::DragonsLair => (256, 4096),
        MapIdentifier::CorruptedCastle => (3264, 8224),
        MapIdentifier::CastleMonillud => (1664, 5696),
        MapIdentifier::ThroneRoomConfrontation => (3456, 6976),
        MapIdentifier::TheUnderworld => (5760, 1600),
        MapIdentifier::Otherworld => (5760, 3648),
        MapIdentifier::MoltenCavern => (4736, 0),
        MapIdentifier::TheDungeons => (1664, 6976),
        MapIdentifier::ItemShop => (6528, 4672),
        MapIdentifier::Convergence => (1280, 4480),
        MapIdentifier::TrialOfReality => (0, 4480),
        MapIdentifier::HauntedManse => (1280, 1536),
        MapIdentifier::SmugglersRoad => (6784, 4160),
        MapIdentifier::SmugglersRuin => (6784, 3648),
        MapIdentifier::Unknown => (0, 0),
    }
}
