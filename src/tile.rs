use std::ops::{Deref, DerefMut};

use image::{ImageBuffer, Pixel};

use crate::data::TILE_16S;

#[derive(Default)]
pub struct Tile8 {
    index: u16,
    flipx: bool,
    flipy: bool,
    rotate: bool,
}
pub type Tile16 = [Tile8; 4];

pub fn tile16_list() -> Vec<Tile16> {
    let mut tile16_list: Vec<Tile16> = TILE_16S.iter().map(|&tile8s|[
        Tile8 { index: tile8s[0], ..Tile8::default() },
        Tile8 { index: tile8s[1], ..Tile8::default() },
        Tile8 { index: tile8s[2], ..Tile8::default() },
        Tile8 { index: tile8s[3], ..Tile8::default() },
    ]).collect::<Vec<_>>();
    tile16_list[3] = [
        Tile8 { index: 239, ..Tile8::default() },
        Tile8 { index: 239, flipx: true, ..Tile8::default() },
        Tile8 { index: 239, flipy: true, ..Tile8::default() },
        Tile8 { index: 239, flipx: true, flipy: true, ..Tile8::default() },
    ];
    tile16_list[6] = [
        Tile8 { index: 92, ..Tile8::default() },
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
        Tile8 { index: 93, ..Tile8::default() },
        Tile8 { index: 93, flipx: true, ..Tile8::default() },
        Tile8 { index: 93, flipy: true, ..Tile8::default() },
        Tile8 { index: 93, flipx: true, flipy: true, ..Tile8::default() },
    ];
    tile16_list
}

pub type Tile8Data = Vec<Vec<u8>>;
pub fn draw_tile16<P, Container>(
    index: usize,
    tile8_list: &[Tile8Data],
    tile16_list: &[Tile16],
    palette: [P; 4],
    image: &mut ImageBuffer<P, Container>,
    xoffset: u32,
    yoffset: u32
)
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    if let Some(tile16) = tile16_list.get(index - 1) {
        for tile_index in 0..4 {
            let tile_xoffset = xoffset + if tile_index % 2 == 1 { 8 } else { 0 };
            let tile_yoffset = yoffset + if tile_index > 1 { 8 } else { 0 };
            let Tile8 {
                index,
                flipx,
                flipy,
                rotate,
            } = &tile16[tile_index];
            let tile8 = &tile8_list[*index as usize];

            for (y, row) in tile8.into_iter().enumerate() {
                for (x, pixel) in row.into_iter().enumerate() {
                    let pixel = palette[*pixel as usize];
                    let mut x =
                    if *flipx { 7 - x }
                    else { x } as u32;
                    let mut y =
                    if *flipy { 7 - y }
                    else { y } as u32;
                    if *rotate {
                        let temp = x;
                        x = y;
                        y = temp;
                    }
                    image.put_pixel(x + tile_xoffset, y + tile_yoffset, pixel);
                }
            }
        }
    }
}
