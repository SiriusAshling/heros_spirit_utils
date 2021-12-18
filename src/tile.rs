use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Pixel};

use crate::data::{TILE_16S, SPRITE_TILE_BIT_TABLE, SPRITE_TILE_FLIP_TABLE, SPRITE_TILE_BITS, SPRITE_TILE_FLIPS};

#[derive(Default)]
pub struct Tile8 {
    pub index: u16,
    pub flipx: bool,
    pub flipy: bool,
    pub rotate: bool,
}

impl From<u16> for Tile8 {
    fn from(index: u16) -> Tile8 {
        Tile8 { index, ..Tile8::default() }
    }
}

pub type Tile16 = [Tile8; 4];

fn build_tile(id: usize) -> Tile16 {
    let bit_index = SPRITE_TILE_BIT_TABLE[id] as usize * 4;
    let flip_index = SPRITE_TILE_FLIP_TABLE[id] as usize * 12;

    let build_tile_8 = |tile_index| {
        let by_three = tile_index * 3;

        let index = SPRITE_TILE_BITS[bit_index + tile_index];
        let flipx = SPRITE_TILE_FLIPS[flip_index + by_three];
        let flipy = SPRITE_TILE_FLIPS[flip_index + by_three + 1];
        let rotate = SPRITE_TILE_FLIPS[flip_index + by_three + 2];
        Tile8 { index, flipx, flipy, rotate }
    };

    [build_tile_8(0), build_tile_8(1), build_tile_8(2), build_tile_8(3)]
}

pub fn sprite_tile16_list() -> Vec<Tile16> {
    let mut tile16_list = Vec::with_capacity(113);

    for index in 0..=113 {
        tile16_list.push(build_tile(index));
    }

    tile16_list
}

pub fn map_tile16_list() -> Vec<Tile16> {
    let mut tile16_list: Vec<Tile16> = TILE_16S.iter().map(|&tile8s|[
        Tile8::from(tile8s[0]),
        Tile8::from(tile8s[1]),
        Tile8::from(tile8s[2]),
        Tile8::from(tile8s[3]),
    ]).collect::<Vec<_>>();
    tile16_list[3] = [
        Tile8::from(239),
        Tile8 { index: 239, flipx: true, ..Tile8::default() },
        Tile8 { index: 239, flipy: true, ..Tile8::default() },
        Tile8 { index: 239, flipx: true, flipy: true, ..Tile8::default() },
    ];
    tile16_list[6] = [
        Tile8::from(92),
        Tile8 { index: 92, flipx: true, ..Tile8::default() },
        Tile8 { index: 92, flipy: true, ..Tile8::default() },
        Tile8 { index: 92, flipx: true, flipy: true, ..Tile8::default() },
    ];
    tile16_list[26] = [
        Tile8 { index: 81, flipx: true, flipy: true, rotate: true },
        Tile8 { index: 65, flipx: true, flipy: true, rotate: true },
        Tile8 { index: 80, flipx: true, flipy: true, rotate: true },
        Tile8 { index: 64, flipx: true, flipy: true, rotate: true },
    ];
    tile16_list[28] = [
        Tile8::from(93),
        Tile8 { index: 93, flipx: true, ..Tile8::default() },
        Tile8 { index: 93, flipy: true, ..Tile8::default() },
        Tile8 { index: 93, flipx: true, flipy: true, ..Tile8::default() },
    ];
    tile16_list
}

pub fn draw_tile8<P, Container>(
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

pub type Tile8Data = Vec<Vec<u8>>;
pub fn draw_tile16<P, Container>(
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
