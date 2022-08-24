use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::path::Path;

use image::{RgbaImage, ImageFormat, ImageBuffer, Pixel, Rgba};

use crate::graphics::{Tile8, TileData, Tile16, Tile8Data};
use crate::palette::{DEFAULT_PALETTE, self};
use crate::data::{TERRAIN_FLAGS, BRIGHT_MAPS};
use crate::sprite::{Sprite, Collectible, Enemy};
use crate::map::{MapIdentifier, Map};
use crate::error::SimpleError;

const TILE8_ROW_LENGTH: u32 = 16;

fn draw_tile8<P, Container>(
    tile8_list: &[Tile8Data],
    tile8: &Tile8,
    palette: [P; 4],
    image: &mut ImageBuffer<P, Container>,
    xoffset: u32,
    yoffset: u32,
    blend: bool,
)
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    let Tile8 {
        index,
        flipx,
        flipy,
        rotate,
    } = tile8;
    let tile8 = &tile8_list[*index as usize];

    for (y, row) in tile8.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let pixel = palette[*pixel as usize];
            let mut x =
            if *flipx { 7 - x }
            else { x } as u32;
            let mut y =
            if *flipy { 7 - y }
            else { y } as u32;
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
)
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    for (tile_index, tile8) in tile16.iter().enumerate() {
        let tile_xoffset = xoffset + if tile_index % 2 == 1 { 8 } else { 0 };
        let tile_yoffset = yoffset + if tile_index > 1 { 8 } else { 0 };

        draw_tile8(tile8_list, tile8, palette, image, tile_xoffset, tile_yoffset, blend);
    }
}

pub fn draw_tile8s(path: impl AsRef<Path>, tile8_list: &[Tile8Data]) -> Result<(), Box<dyn Error>> {
    let len = tile8_list.len() as u32;
    let width = TILE8_ROW_LENGTH * 8;
    let height = (len + 9) / TILE8_ROW_LENGTH * 8;
    let mut image: RgbaImage = ImageBuffer::new(width, height);

    for index in 0..len {
        let xoffset = index % TILE8_ROW_LENGTH * 8;
        let yoffset = index / TILE8_ROW_LENGTH * 8;

        let tile8 = Tile8 { index: index as u16, ..Tile8::default() };

        draw_tile8(tile8_list, &tile8, DEFAULT_PALETTE, &mut image, xoffset, yoffset, false);
    }

    image.save(path)?;

    Ok(())
}

pub fn undraw_tile8s(path: impl AsRef<Path>) -> Result<Vec<Tile8Data>, Box<dyn Error>> {
    let image = image::open(path)?;
    let tile8_list = image.into_rgba8().pixels().collect::<Vec<_>>().chunks(8 * TILE8_ROW_LENGTH as usize).collect::<Vec<_>>().chunks(8).flat_map(|tile8_row| {
        (0..TILE8_ROW_LENGTH as usize).map(|index|
            tile8_row.iter().map(|pixel_row|
                pixel_row[index * 8..(index + 1) * 8].into_iter().map(|pixel|
                    DEFAULT_PALETTE.iter().enumerate().find_map(|(index, default_pixel)|
                        if *pixel == default_pixel { Some(index as u8) } else { None }
                    ).ok_or(SimpleError("Invalid pixel color in tile8s"))
                ).collect()
            ).collect()
        )
    }).collect::<Result<_, _>>()?;

    Ok(tile8_list)
}

pub fn draw_tile16s(path: impl AsRef<Path>, tile_data: &TileData) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref().to_owned();

    for (index, tile16) in tile_data.map_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        draw_tile16(&tile_data.tile8_list, tile16, DEFAULT_PALETTE, &mut image, 0, 0, false);

        let mut tile16_path = path.clone();
        tile16_path.push(format!("map_{}.png", index + 1));
        image.save_with_format(&tile16_path, ImageFormat::Png)?;
    }
    for (index, tile16) in tile_data.sprite_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        let palette = palette::get_sprite_palette(index);
        draw_tile16(&tile_data.tile8_list, tile16, palette, &mut image, 0, 0, false);

        let mut tile16_path = path.clone();
        tile16_path.push(format!("sprite_{}.png", index));
        image.save_with_format(&tile16_path, ImageFormat::Png)?;
    }
    for (index, tile16) in tile_data.enemy_tile16_list.iter().enumerate() {
        let mut image: RgbaImage = ImageBuffer::new(16, 16);

        let palette = palette::get_enemy_palette(index);
        draw_tile16(&tile_data.tile8_list, tile16, palette, &mut image, 0, 0, false);

        let mut tile16_path = path.clone();
        tile16_path.push(format!("enemy_{}.png", index));
        image.save_with_format(&tile16_path, ImageFormat::Png)?;
    }

    Ok(())
}

pub fn draw_map(map: Map, tile_data: &TileData) -> RgbaImage {
    let width = map.tiles[0].len();
    let height = map.tiles.len();
    let mut image: RgbaImage = ImageBuffer::new(width as u32 * 16, height as u32 * 16);

    for (y, row) in map.tiles.into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            draw_tile_onto(tile, x as u32, y as u32, map.identifier, tile_data, &mut image);
        }
    }

    for (y, row) in map.sprites.into_iter().enumerate() {
        for (x, sprite) in row.into_iter().enumerate() {
            if let Some(sprite) = sprite {
                draw_sprite_onto(sprite.kind.into(), x as u32, y as u32, map.identifier, tile_data, &mut image);
            }
        }
    }

    image
}

fn draw_tile_onto(tile: u8, x: u32, y: u32, map_id: MapIdentifier, tile_data: &TileData, image: &mut RgbaImage) {
    let mut tile = tile as usize;

    let tile_flags = if map_id == MapIdentifier::Glitch {
        tile += 67;
        TERRAIN_FLAGS[tile - 64]
    } else {
        TERRAIN_FLAGS[tile]
    };

    if let Some(tile16) = tile_data.map_tile16_list.get(tile - 1) {
        let pixel_x = x * 16;
        let pixel_y = y * 16;

        let passage = tile_flags & 0b00010010 == 0b00010010;
        let palette = palette::get_map_palette(tile, map_id);

        draw_tile16(&tile_data.tile8_list, tile16, palette, image, pixel_x, pixel_y, false);

        if passage {
            draw_tile8(&tile_data.tile8_list, &Tile8::from(825), DEFAULT_PALETTE, image, pixel_x + 4, pixel_y + 4, false);
        }
    }
}

fn draw_sprite_onto(sprite: Sprite, x: u32, y: u32, map_id: MapIdentifier, tile_data: &TileData, image: &mut RgbaImage) {
    let pixel_x = x * 16;
    let pixel_y = y * 16;

    if let Sprite::Enemy(enemy) = sprite {
        let sprite_index = match enemy {
            Enemy::GDragon => {
                let palette = match map_id {
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
                        draw_tile8(&tile_data.tile8_list, &tile8, palette, image, xoffset, yoffset, true);
                        index += 1;
                    }
                }

                return;
            },
            Enemy::Basilisk => {
                let palette = palette::lookup_palette([7, 22, 23, 64]);
                let mut index = 582;
                for tile8_y in 0..4 {
                    for tile8_x in 0..2 {
                        let tile8 = Tile8::from(index);
                        let xoffset = pixel_x + tile8_x * 8;
                        let yoffset = pixel_y + tile8_y * 8;
                        draw_tile8(&tile_data.tile8_list, &tile8, palette, image, xoffset, yoffset, true);
                        index += 1;
                    }
                }

                return;
            },
            // Enemy::Ragnarok => 20,
            // Enemy::EvilBunny => 42,
            // Enemy::DarkGhost => 43,
            _ => enemy as usize,
        };

        let tile16 = &tile_data.enemy_tile16_list[sprite_index];
        let palette = palette::get_enemy_palette(sprite_index);
        draw_tile16(&tile_data.tile8_list, tile16, palette, image, pixel_x, pixel_y, true);
        return;
    }

    let sprite_index = match sprite {
        Sprite::Collectible(collectible) => match collectible {
            Collectible::Sword | Collectible::SilverKey if BRIGHT_MAPS.contains(&(map_id as u8)) => collectible as usize + 18,
            Collectible::PossumCoin => 100,
            _ => collectible as usize + 17,
        },
        Sprite::Door(door) => door as usize + 2,
        Sprite::WindRoute => 67,
        Sprite::Save => 0,
        _ => usize::MAX,
    };
    if sprite_index == usize::MAX { return }

    let tile16 = &tile_data.sprite_tile16_list[sprite_index];
    let palette = palette::get_sprite_palette(sprite_index);
    draw_tile16(&tile_data.tile8_list, tile16, palette, image, pixel_x, pixel_y, true);
}

pub fn draw_tile(tile: u8, map_id: MapIdentifier, tile_data: &TileData) -> RgbaImage {
    let mut image = RgbaImage::new(16, 16);
    draw_tile_onto(tile, 0, 0, map_id, tile_data, &mut image);
    image
}

pub fn draw_sprite(sprite: u8, map_id: MapIdentifier, tile_data: &TileData) -> RgbaImage {
    let sprite = Sprite::from(sprite);
    let (width, height) = sprite.tile_size();
    let mut image = RgbaImage::new(width as u32 * 16, height as u32 * 16);
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
