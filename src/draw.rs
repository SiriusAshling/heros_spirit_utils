use std::ops::{Deref, DerefMut};
use std::path::Path;

use image::{ImageBuffer, ImageFormat, Pixel, Rgba, RgbaImage};

use crate::data::{BRIGHT_MAPS, MAP_PALETTES, TERRAIN_FLAGS};
use crate::graphics::{Tile16, Tile8, Tile8Data, TileData};
use crate::map::{self, Map};
use crate::palette::{self, DEFAULT_PALETTE};
use crate::sprite::{Collectible, Door, Enemy, Sprite, Things};
use crate::Result;

const TILE8_ROW_LENGTH: u32 = 16;

fn draw_tile8<P, Container>(
    tile8_list: &[Tile8Data],
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
    let tile8 = &tile8_list[*index as usize];

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
    tile8_list: &[Tile8Data],
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

        draw_tile8(
            tile8_list,
            tile8,
            palette,
            image,
            tile_xoffset,
            tile_yoffset,
            blend,
        );
    }
}

pub fn draw_tile8s(path: impl AsRef<Path>, tile8_list: &[Tile8Data]) -> Result<()> {
    let len = tile8_list.len() as u32;
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

        draw_tile8(
            tile8_list,
            &tile8,
            DEFAULT_PALETTE,
            &mut image,
            xoffset,
            yoffset,
            false,
        );
    }

    image.save(path)?;

    Ok(())
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

pub fn draw_tile16s(tile_data: &TileData) -> Result<()> {
    for (index, tile16) in tile_data.map_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        draw_tile16(
            &tile_data.tile8_list,
            tile16,
            DEFAULT_PALETTE,
            &mut image,
            0,
            0,
            false,
        );

        let path = format!("rom_files/Graphics/tile16/map_{}.png", index + 1);
        image.save_with_format(&path, ImageFormat::Png)?;
    }
    for (index, tile16) in tile_data.sprite_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        let palette = palette::get_sprite_palette(index, 0);
        draw_tile16(
            &tile_data.tile8_list,
            tile16,
            palette,
            &mut image,
            0,
            0,
            false,
        );

        let path = format!("rom_files/Graphics/tile16/sprite_{index}.png");
        image.save_with_format(&path, ImageFormat::Png)?;
    }
    for (index, tile16) in tile_data.enemy_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        let palette = palette::get_enemy_palette(index);
        draw_tile16(
            &tile_data.tile8_list,
            tile16,
            palette,
            &mut image,
            0,
            0,
            false,
        );

        let path = format!("rom_files/Graphics/tile16/enemy_{index}.png");
        image.save_with_format(&path, ImageFormat::Png)?;
    }

    Ok(())
}

pub fn draw_map(map: Map, tile_data: &TileData) -> RgbaImage {
    let width = map.tiles[0].len();
    let height = map.tiles.len();
    let mut image: RgbaImage = ImageBuffer::new(width as u32 * 16, height as u32 * 16);

    for (y, row) in map.tiles.into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            draw_tile_onto(
                tile,
                x as u32,
                y as u32,
                map.identifier,
                tile_data,
                &mut image,
            );
        }
    }

    for (y, row) in map.sprites.into_iter().enumerate() {
        for (x, sprite) in row.into_iter().enumerate() {
            if let Some(sprite) = sprite {
                draw_sprite_onto(
                    sprite.kind.into(),
                    x as u32,
                    y as u32,
                    map.identifier,
                    tile_data,
                    &mut image,
                );
            }
        }
    }

    image
}

fn draw_tile_onto(
    tile: u8,
    x: u32,
    y: u32,
    map_id: u8,
    tile_data: &TileData,
    image: &mut RgbaImage,
) {
    let mut tile = tile as usize;

    let tile_flags = if map_id == map::GLITCH {
        tile += 67;
        TERRAIN_FLAGS[tile - 64]
    } else {
        TERRAIN_FLAGS[tile]
    };

    if let Some(tile16) = tile_data.map_tile16_list.get(tile - 1) {
        let pixel_x = x * 16;
        let pixel_y = y * 16;

        let passage = tile_flags & 0b0001_0010 == 0b0001_0010;
        let palette = palette::get_map_palette(tile, map_id);

        draw_tile16(
            &tile_data.tile8_list,
            tile16,
            palette,
            image,
            pixel_x,
            pixel_y,
            false,
        );

        if passage {
            draw_tile8(
                &tile_data.tile8_list,
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
fn draw_map_sprite_tile_onto(
    tile: u8,
    x: u32,
    y: u32,
    map_id: u8,
    tile_data: &TileData,
    image: &mut RgbaImage,
) {
    let index = (tile * 29 + MAP_PALETTES[map_id as usize]) as usize;

    if let Some(tile16) = tile_data.map_sprite_tile16_list.get(index) {
        let pixel_x = x * 16;
        let pixel_y = y * 16;

        let palette = palette::get_map_sprite_palette(u8::from(tile == 1), index);

        draw_tile16(
            &tile_data.tile8_list,
            tile16,
            palette,
            image,
            pixel_x,
            pixel_y,
            false,
        );
    }
}

fn draw_sprite_onto(
    sprite: Sprite,
    x: u32,
    y: u32,
    map_id: u8,
    tile_data: &TileData,
    image: &mut RgbaImage,
) {
    let pixel_x = x * 16;
    let pixel_y = y * 16;

    let is_enemy = matches!(sprite, Sprite::Enemy(_));

    let sprite_index = match sprite {
        Sprite::Enemy(enemy) => match enemy {
            Enemy::GDragon => {
                draw_gdragon_onto(pixel_x, pixel_y, map_id, tile_data, image);
                return;
            }
            Enemy::Basilisk => {
                draw_basilisk_onto(pixel_x, pixel_y, tile_data, image);
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
                draw_tile_onto(16, x, y, map_id, tile_data, image);
                30
            }
            Things::NGMountain => {
                draw_tile_onto(16, x, y, map_id, tile_data, image);
                21
            }
            Things::NGPMountain => {
                draw_tile_onto(16, x, y, map_id, tile_data, image);
                109
            }
            Things::NGPBoulder => {
                draw_sprite_onto(Sprite::Door(Door::Boulder), x, y, map_id, tile_data, image);
                109
            }
            Things::NGPWall => {
                draw_map_sprite_tile_onto(3, x, y, map_id, tile_data, image);
                109
            }
            Things::NGPTransfer => {
                draw_tile_onto(7, x, y, map_id, tile_data, image);
                109
            }
            Things::UnderworldKeyhole => 84,
        },
        Sprite::Other(_) => return,
    };

    let tile16 = &if is_enemy {
        &tile_data.enemy_tile16_list
    } else {
        &tile_data.sprite_tile16_list
    }[sprite_index];
    let palette = if is_enemy {
        palette::get_enemy_palette(sprite_index)
    } else {
        palette::get_sprite_palette(sprite_index, map_id)
    };
    draw_tile16(
        &tile_data.tile8_list,
        tile16,
        palette,
        image,
        pixel_x,
        pixel_y,
        true,
    );
}

fn draw_gdragon_onto(
    pixel_x: u32,
    pixel_y: u32,
    map_id: u8,
    tile_data: &TileData,
    image: &mut RgbaImage,
) {
    let palette = match map_id {
        map::CASTLE_MONILLUD => palette::lookup_palette([11, 9, 25, 64]),
        map::THE_UNDERWORLD => palette::lookup_palette([7, 22, 23, 64]),
        _ => palette::lookup_palette([12, 18, 16, 64]),
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
            draw_tile8(
                &tile_data.tile8_list,
                &tile8,
                palette,
                image,
                xoffset,
                yoffset,
                true,
            );
            index += 1;
        }
    }
}

fn draw_basilisk_onto(pixel_x: u32, pixel_y: u32, tile_data: &TileData, image: &mut RgbaImage) {
    let palette = palette::lookup_palette([7, 22, 23, 64]);
    let mut index = 582;
    for tile8_y in 0..4 {
        for tile8_x in 0..2 {
            let tile8 = Tile8::from(index);
            let xoffset = pixel_x + tile8_x * 8;
            let yoffset = pixel_y + tile8_y * 8;
            draw_tile8(
                &tile_data.tile8_list,
                &tile8,
                palette,
                image,
                xoffset,
                yoffset,
                true,
            );
            index += 1;
        }
    }
}

pub fn draw_tile(tile: u8, map_id: u8, tile_data: &TileData) -> RgbaImage {
    let mut image = RgbaImage::new(16, 16);
    draw_tile_onto(tile, 0, 0, map_id, tile_data, &mut image);
    image
}

pub fn draw_sprite(sprite: u8, map_id: u8, tile_data: &TileData) -> RgbaImage {
    let sprite = Sprite::from(sprite);
    let (width, height) = sprite.tile_size();
    let mut image = RgbaImage::new(u32::from(width) * 16, u32::from(height) * 16);
    draw_sprite_onto(sprite, 0, 0, map_id, tile_data, &mut image);
    draw_sprite_frame(&mut image);
    image
}

fn draw_sprite_frame(image: &mut RgbaImage) {
    for pos in 0..16 {
        image.put_pixel(pos, 0, Rgba([0, 0, 0, 255]));
        image.put_pixel(pos, 15, Rgba([0, 0, 0, 255]));
        image.put_pixel(0, pos, Rgba([0, 0, 0, 255]));
        image.put_pixel(15, pos, Rgba([0, 0, 0, 255]));
    }
}

pub fn merge_maps(maps: Vec<(u8, RgbaImage)>) -> RgbaImage {
    let mut image = RgbaImage::new(7808, 9008);

    for (identifier, map) in maps {
        let (x_offset, mut y) = map_offset(identifier);
        if x_offset + y == 0 {
            continue;
        }

        for row in map.rows() {
            let mut x = x_offset;
            for pixel in row {
                image.put_pixel(x, y, *pixel);
                x += 1;
            }
            y += 1;
        }
    }

    image
}

fn map_offset(identifier: u8) -> (u32, u32) {
    match identifier {
        map::DUST_SHELF => (640, 3968),
        map::THRONE_ROOM => (3840, 6976),
        map::EXPLODING_THRONE_ROOM => (2944, 5696),
        map::CASTLE_RUINS => (4224, 5696),
        map::NORTH_MUNDEMAN => (3712, 1600),
        map::SOUTH_MUNDEMAN => (3712, 2624),
        map::VERDANT_COAST => (3712, 3648),
        map::OTHERWORLD_ARENA => (5760, 4672),
        map::CASTLE_GROUNDS => (1664, 3648),
        map::SANCTUARY => (1152, 3968),
        map::THE_TUNNELS => (2016, 3264),
        map::GLITCH => (3264, 7360),
        map::LUDDERSHORE => (4736, 1600),
        map::THE_TUNDRA => (1664, 1216),
        map::FROZEN_SHORE => (3712, 576),
        map::HALLOW_GROUND => (640, 1920),
        map::SOUTHERN_SWAMP => (640, 2944),
        map::DRAGONS_LAIR => (256, 4096),
        map::CORRUPTED_CASTLE => (3264, 8224),
        map::CASTLE_MONILLUD => (1664, 5696),
        map::THRONE_ROOM_CONFRONTATION => (3456, 6976),
        map::THE_UNDERWORLD => (5760, 1600),
        map::OTHERWORLD => (5760, 3648),
        map::MOLTEN_CAVERN => (4736, 0),
        map::THE_DUNGEONS => (1664, 6976),
        map::ITEM_SHOP => (6528, 4672),
        map::CONVERGENCE => (1280, 4480),
        map::TRIAL_OF_REALITY => (0, 4480),
        map::HAUNTED_MANSE => (1280, 1536),
        map::SMUGGLERS_ROAD => (6784, 4160),
        map::SMUGGLERS_RUIN => (6784, 3648),
        _ => (0, 0),
    }
}
