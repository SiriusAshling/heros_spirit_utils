use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::Path;

use image::{ImageBuffer, ImageFormat, Pixel, Rgba, RgbaImage};

use crate::data::{BRIGHT_MAPS, TERRAIN_FLAGS};
use crate::graphics::{Tile16, Tile8, Tile8Data, TileData, DEFAULT_PALETTE};
use crate::map::{Collectible, Door, Enemy, Map, MapColors, MapMeta, Sprite, Things};
use crate::Result;

use super::{get_enemy_palette, get_sprite_palette, lookup_palette};

const TILE8_ROW_LENGTH: u32 = 16;

pub struct DrawData<'a> {
    pub tile_data: &'a TileData,
    pub map_colors: &'a MapColors,
    pub map_meta: &'a HashMap<usize, MapMeta>,
}

impl DrawData<'_> {
    pub fn draw_map(&self, map: &Map) -> RgbaImage {
        let width = map.tiles[0].len();
        let height = map.tiles.len();
        let mut image: RgbaImage = ImageBuffer::new(width as u32 * 16, height as u32 * 16);

        for (x, y, tile) in map.tiles_with_positions() {
            self.draw_tile_onto(tile, x as u32, y as u32, map.identifier, &mut image);
        }

        for (x, y, sprite) in map.sprites_with_positions() {
            self.draw_sprite_onto(
                sprite.kind.into(),
                x as u32,
                y as u32,
                map.identifier,
                &mut image,
            );
        }

        image
    }

    pub fn draw_tile(&self, tile: u8, map_id: u8) -> RgbaImage {
        let mut image = RgbaImage::new(16, 16);
        self.draw_tile_onto(tile, 0, 0, map_id, &mut image);
        image
    }

    pub fn draw_sprite(&self, sprite: u8, map_id: u8) -> RgbaImage {
        let sprite = Sprite::from(sprite);
        let (width, height) = sprite.tile_size();
        let mut image = RgbaImage::new(u32::from(width) * 16, u32::from(height) * 16);
        self.draw_sprite_onto(sprite, 0, 0, map_id, &mut image);
        draw_sprite_frame(&mut image);
        image
    }

    pub fn draw_tile8s(&self, path: impl AsRef<Path>) -> Result<()> {
        let len = self.tile_data.tile8_list.len() as u32;
        let width = TILE8_ROW_LENGTH * 8;
        let height = (len + 9) / TILE8_ROW_LENGTH * 8;
        let mut image: RgbaImage = ImageBuffer::new(width, height);

        for index in 0..len {
            let xoffset = index % TILE8_ROW_LENGTH * 8;
            let yoffset = index / TILE8_ROW_LENGTH * 8;

            let tile8 = Tile8 {
                index: index as u16,
                ..Tile8::default()
            };

            self.draw_tile8(&tile8, DEFAULT_PALETTE, &mut image, xoffset, yoffset, false);
        }

        image.save(path)?;

        Ok(())
    }

    pub fn draw_tile16s(&self) -> Result<()> {
        for (index, tile16) in self.tile_data.map_tile16_list.iter().enumerate() {
            let mut image: RgbaImage = ImageBuffer::new(16, 16);

            self.draw_tile16(tile16, DEFAULT_PALETTE, &mut image, 0, 0, false);

            let path = format!("rom_files/Graphics/tile16/map_{}.png", index + 1);
            image.save_with_format(&path, ImageFormat::Png)?;
        }
        for (index, tile16) in self.tile_data.sprite_tile16_list.iter().enumerate() {
            let mut image: RgbaImage = ImageBuffer::new(16, 16);

            let palette = get_sprite_palette(index, 0);
            self.draw_tile16(tile16, palette, &mut image, 0, 0, false);

            let path = format!("rom_files/Graphics/tile16/sprite_{index}.png");
            image.save_with_format(&path, ImageFormat::Png)?;
        }
        for (index, tile16) in self.tile_data.enemy_tile16_list.iter().enumerate() {
            let mut image: RgbaImage = ImageBuffer::new(16, 16);

            let palette = get_enemy_palette(index);
            self.draw_tile16(tile16, palette, &mut image, 0, 0, false);

            let path = format!("rom_files/Graphics/tile16/enemy_{index}.png");
            image.save_with_format(&path, ImageFormat::Png)?;
        }

        Ok(())
    }

    fn draw_tile_onto(&self, tile: u8, x: u32, y: u32, map_id: u8, image: &mut RgbaImage) {
        let tile = tile as usize;
        let tile_flags = TERRAIN_FLAGS[tile];

        // TODO check if this still exists, it panics on tileset creation in glitch since a sprite uses a high tile id
        // let tile_flags = if map_id == map::GLITCH {
        //     tile += 67;
        //     TERRAIN_FLAGS[tile - 64]
        // } else {
        //     TERRAIN_FLAGS[tile]
        // };

        // TODO this underflows sometimes
        let index = tile.overflowing_sub(1).0;
        if let Some(tile16) = self.tile_data.map_tile16_list.get(index) {
            let pixel_x = x * 16;
            let pixel_y = y * 16;

            let passage = tile_flags & 0b0001_0010 == 0b0001_0010;
            let palette = self.get_map_palette(tile, map_id);

            self.draw_tile16(tile16, palette, image, pixel_x, pixel_y, false);

            if passage {
                self.draw_tile8(
                    &Tile8::from(825),
                    DEFAULT_PALETTE,
                    image,
                    pixel_x + 4,
                    pixel_y + 4,
                    false,
                );
            }
        }
    }

    fn draw_sprite_onto(&self, sprite: Sprite, x: u32, y: u32, map_id: u8, image: &mut RgbaImage) {
        let pixel_x = x * 16;
        let pixel_y = y * 16;

        let is_enemy = matches!(sprite, Sprite::Enemy(_));

        let sprite_index = match sprite {
            Sprite::Enemy(enemy) => match enemy {
                Enemy::GDragon => {
                    self.draw_gdragon_onto(pixel_x, pixel_y, map_id, image);
                    return;
                }
                Enemy::Basilisk => {
                    self.draw_basilisk_onto(pixel_x, pixel_y, image);
                    return;
                }
                // Enemy::Ragnarok => 20,
                // Enemy::EvilBunny => 42,
                // Enemy::DarkGhost => 43,
                _ => enemy as usize,
            },
            Sprite::Collectible(collectible) => match collectible {
                Collectible::Sword | Collectible::SilverKey if BRIGHT_MAPS.contains(&map_id) => {
                    collectible as usize + 18
                }
                Collectible::PossumCoin => 100,
                _ => collectible as usize + 17,
            },
            Sprite::Gear(gear) => gear as usize + 17,
            Sprite::Door(door) => door as usize + 2,
            Sprite::WindRoute => 67,
            Sprite::Save => 0,
            Sprite::Things(things) => match things {
                Things::CompassWall => {
                    self.draw_tile_onto(16, x, y, map_id, image);
                    30
                }
                Things::NGMountain => {
                    self.draw_tile_onto(16, x, y, map_id, image);
                    21
                }
                Things::NGPMountain => {
                    self.draw_tile_onto(16, x, y, map_id, image);
                    109
                }
                Things::NGPBoulder => {
                    self.draw_sprite_onto(Sprite::Door(Door::Boulder), x, y, map_id, image);
                    109
                }
                Things::NGPWall => {
                    // TODO this is actually a map sprite tile
                    self.draw_tile_onto(65, x, y, map_id, image);
                    109
                }
                Things::NGPTransfer => {
                    self.draw_tile_onto(7, x, y, map_id, image);
                    109
                }
                Things::UnderworldKeyhole => 84,
                Things::Transfer | Things::Warp => return,
            },
            Sprite::Other(_) => return,
        };

        let tile16 = &if is_enemy {
            &self.tile_data.enemy_tile16_list
        } else {
            &self.tile_data.sprite_tile16_list
        }[sprite_index];
        let palette = if is_enemy {
            get_enemy_palette(sprite_index)
        } else {
            get_sprite_palette(sprite_index, map_id)
        };
        self.draw_tile16(tile16, palette, image, pixel_x, pixel_y, true);
    }

    fn draw_gdragon_onto(&self, pixel_x: u32, pixel_y: u32, map_id: u8, image: &mut RgbaImage) {
        let palette = match map_id {
            Map::CASTLE_MONILLUD => lookup_palette([11, 9, 25, 64]),
            Map::THE_UNDERWORLD => lookup_palette([7, 22, 23, 64]),
            _ => lookup_palette([12, 18, 16, 64]),
        };

        let mut index = 561;
        for tile8_y in 0..4 {
            for tile8_x in 0..6 {
                if tile8_y == 0 && (tile8_x == 0 || tile8_x > 3) {
                    continue;
                }
                let tile8 = Tile8::from(index);
                let xoffset = pixel_x + tile8_x * 8;
                let yoffset = pixel_y + tile8_y * 8;
                self.draw_tile8(&tile8, palette, image, xoffset, yoffset, true);
                index += 1;
            }
        }
    }

    fn draw_basilisk_onto(&self, pixel_x: u32, pixel_y: u32, image: &mut RgbaImage) {
        let palette = lookup_palette([7, 22, 23, 64]);
        let mut index = 582;
        for tile8_y in 0..4 {
            for tile8_x in 0..2 {
                let tile8 = Tile8::from(index);
                let xoffset = pixel_x + tile8_x * 8;
                let yoffset = pixel_y + tile8_y * 8;
                self.draw_tile8(&tile8, palette, image, xoffset, yoffset, true);
                index += 1;
            }
        }
    }

    fn draw_tile8<P, Container>(
        &self,
        tile8: &Tile8,
        palette: [P; 4],
        image: &mut ImageBuffer<P, Container>,
        xoffset: u32,
        yoffset: u32,
        blend: bool,
    ) where
        P: Pixel + 'static,
        P::Subpixel: 'static,
        Container: Deref<Target = [P::Subpixel]> + DerefMut,
    {
        let Tile8 {
            index,
            flip_x,
            flip_y,
            rotate,
        } = tile8;
        let tile8 = &self.tile_data.tile8_list[*index as usize];

        for (y, row) in tile8.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                let pixel = palette[*pixel as usize];
                let mut x = if *flip_x { 7 - x } else { x } as u32;
                let mut y = if *flip_y { 7 - y } else { y } as u32;
                if *rotate {
                    std::mem::swap(&mut x, &mut y);
                }
                x += xoffset;
                y += yoffset;

                if blend {
                    image.get_pixel_mut(x, y).blend(&pixel);
                } else {
                    image.put_pixel(x, y, pixel);
                }
            }
        }
    }

    fn draw_tile16<P, Container>(
        &self,
        tile16: &Tile16,
        palette: [P; 4],
        image: &mut ImageBuffer<P, Container>,
        xoffset: u32,
        yoffset: u32,
        blend: bool,
    ) where
        P: Pixel + 'static,
        P::Subpixel: 'static,
        Container: Deref<Target = [P::Subpixel]> + DerefMut,
    {
        for (tile_index, tile8) in tile16.iter().enumerate() {
            let tile_xoffset = xoffset + if tile_index % 2 == 1 { 8 } else { 0 };
            let tile_yoffset = yoffset + if tile_index > 1 { 8 } else { 0 };

            self.draw_tile8(tile8, palette, image, tile_xoffset, tile_yoffset, blend);
        }
    }
}

pub fn undraw_tile8s(path: impl AsRef<Path>) -> Result<Vec<Tile8Data>> {
    let image = image::open(path)?;
    let tile8_list = image
        .into_rgba8()
        .pixels()
        .collect::<Vec<_>>()
        .chunks(8 * TILE8_ROW_LENGTH as usize)
        .collect::<Vec<_>>()
        .chunks(8)
        .flat_map(|tile8_row| {
            (0..TILE8_ROW_LENGTH as usize).map(|index| {
                tile8_row
                    .iter()
                    .map(|pixel_row| {
                        pixel_row[index * 8..(index + 1) * 8]
                            .iter()
                            .map(|pixel| {
                                let pixel = DEFAULT_PALETTE
                                    .iter()
                                    .enumerate()
                                    .find_map(|(index, default_pixel)| {
                                        if *pixel == default_pixel {
                                            Some(index as u8)
                                        } else {
                                            None
                                        }
                                    })
                                    .ok_or("Invalid pixel color in tile8s")?;
                                Ok(pixel)
                            })
                            .collect()
                    })
                    .collect()
            })
        })
        .collect::<Result<_>>()?;

    Ok(tile8_list)
}

fn draw_sprite_frame(image: &mut RgbaImage) {
    for pos in 0..16 {
        image.put_pixel(pos, 0, Rgba([0, 0, 0, 255]));
        image.put_pixel(pos, 15, Rgba([0, 0, 0, 255]));
        image.put_pixel(0, pos, Rgba([0, 0, 0, 255]));
        image.put_pixel(15, pos, Rgba([0, 0, 0, 255]));
    }
}

pub fn merge_maps(maps: Vec<(u8, RgbaImage)>) -> [(&'static str, RgbaImage); 2] {
    let mut images = [
        ("FullMap.png", RgbaImage::new(7808, 9008)),
        ("HeroicHardcore.png", RgbaImage::new(2048, 2048)),
    ];

    for (identifier, map) in maps {
        let (index, x_offset, mut y) = map_offset(identifier);
        if index == usize::MAX {
            continue;
        }

        let image = &mut images[index].1;

        for row in map.rows() {
            let mut x = x_offset;
            for pixel in row {
                image.put_pixel(x, y, *pixel);
                x += 1;
            }
            y += 1;
        }
    }

    images
}

fn map_offset(identifier: u8) -> (usize, u32, u32) {
    match identifier {
        Map::DUST_SHELF => (0, 640, 3968),
        Map::THRONE_ROOM => (0, 3840, 6976),
        Map::EXPLODING_THRONE_ROOM => (0, 2944, 5696),
        Map::CASTLE_RUINS => (0, 4224, 5696),
        Map::NORTH_MUNDEMAN => (0, 3712, 1600),
        Map::SOUTH_MUNDEMAN => (0, 3712, 2624),
        Map::VERDANT_COAST => (0, 3712, 3648),
        Map::OTHERWORLD_ARENA => (0, 5760, 4672),
        Map::CASTLE_GROUNDS => (0, 1664, 3648),
        Map::SANCTUARY => (0, 1152, 3968),
        Map::THE_TUNNELS => (0, 2016, 3264),
        Map::GLITCH => (0, 3264, 7360),
        Map::LUDDERSHORE => (0, 4736, 1600),
        Map::THE_TUNDRA => (0, 1664, 1216),
        Map::FROZEN_SHORE => (0, 3712, 576),
        Map::HALLOW_GROUND => (0, 640, 1920),
        Map::SOUTHERN_SWAMP => (0, 640, 2944),
        Map::DRAGONS_LAIR => (0, 256, 4096),
        Map::CORRUPTED_CASTLE => (0, 3264, 8224),
        Map::CASTLE_MONILLUD => (0, 1664, 5696),
        Map::THRONE_ROOM_CONFRONTATION => (0, 3456, 6976),
        Map::THE_UNDERWORLD => (0, 5760, 1600),
        Map::OTHERWORLD => (0, 5760, 3648),
        Map::MOLTEN_CAVERN => (0, 4736, 0),
        Map::THE_DUNGEONS => (0, 1664, 6976),
        Map::ITEM_SHOP => (0, 6528, 4672),
        Map::CONVERGENCE => (0, 1280, 4480),
        Map::TRIAL_OF_REALITY => (0, 0, 4480),
        Map::HAUNTED_MANSE => (0, 1280, 1536),
        Map::SMUGGLERS_ROAD => (0, 6784, 4160),
        Map::SMUGGLERS_RUIN => (0, 6784, 3648),
        Map::HHM_CASTLE_GROUNDS => (1, 0, 0),
        Map::HHM_CASTLE_MONILLUD => (1, 1024, 0),
        Map::HHM_STRANGE_AREA => (1, 1024, 1024),
        Map::HHM_THE_UNDERWORLD => (1, 0, 1024),
        _ => (usize::MAX, 0, 0),
    }
}
