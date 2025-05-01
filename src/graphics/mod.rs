mod draw;
mod palette;

pub use draw::{merge_maps, undraw_tile8s, DrawData};
pub use palette::{get_enemy_palette, get_sprite_palette, lookup_palette, DEFAULT_PALETTE};

use crate::data::{
    ENEMY_TILE_BITS, ENEMY_TILE_BIT_TABLE, ENEMY_TILE_FLIPS, SPRITE_TILE_BITS,
    SPRITE_TILE_BIT_TABLE, SPRITE_TILE_FLIPS, SPRITE_TILE_FLIP_TABLE, TILE_16S,
};
use crate::rom::{self, RomReader};
use crate::Result;

#[derive(Debug, PartialEq, Eq)]
pub struct TileData {
    pub tile8_list: Vec<Tile8Data>,
    pub map_tile16_list: Vec<Tile16>,
    pub sprite_tile16_list: Vec<Tile16>,
    pub enemy_tile16_list: Vec<Tile16>,
    // TODO map sprite tiles were supported in a previous version
}

pub type Tile8Data = Vec<Vec<u8>>;

impl TileData {
    pub fn parse(rom: &mut RomReader) -> Result<TileData> {
        let index = rom.index.graphics.ok_or("no graphics.bin in ROM")?;
        let bytes = rom::read_by_index(&mut rom.archive, index)?;
        let tile8_list = decode_graphics(&bytes);
        Ok(Self {
            tile8_list,
            map_tile16_list: map_tile16_list(),
            sprite_tile16_list: sprite_tile16_list(),
            enemy_tile16_list: enemy_tile16_list(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Tile8 {
    pub index: u16,
    pub flip_x: bool,
    pub flip_y: bool,
    pub rotate: bool,
}

impl From<u16> for Tile8 {
    fn from(index: u16) -> Tile8 {
        Tile8 {
            index,
            ..Tile8::default()
        }
    }
}

pub type Tile16 = [Tile8; 4];

fn build_enemy_tile(id: usize) -> Tile16 {
    let bit_index = ENEMY_TILE_BIT_TABLE[id] as usize * 8;
    let flip_index = bit_index * 3;

    let build_tile_8 = |tile_index| {
        let by_three = tile_index * 3;

        let index = ENEMY_TILE_BITS[bit_index + tile_index];
        let flip_x = ENEMY_TILE_FLIPS[flip_index + by_three];
        let flip_y = ENEMY_TILE_FLIPS[flip_index + by_three + 1];
        let rotate = ENEMY_TILE_FLIPS[flip_index + by_three + 2];
        Tile8 {
            index,
            flip_x,
            flip_y,
            rotate,
        }
    };

    [
        build_tile_8(0),
        build_tile_8(1),
        build_tile_8(2),
        build_tile_8(3),
    ]
}

fn build_sprite_tile(id: usize) -> Tile16 {
    let bit_index = SPRITE_TILE_BIT_TABLE[id] as usize * 4;
    let flip_index = SPRITE_TILE_FLIP_TABLE[id] as usize * 12;

    let build_tile_8 = |tile_index| {
        let by_three = tile_index * 3;

        let index = SPRITE_TILE_BITS[bit_index + tile_index];
        let flip_x = SPRITE_TILE_FLIPS[flip_index + by_three];
        let flip_y = SPRITE_TILE_FLIPS[flip_index + by_three + 1];
        let rotate = SPRITE_TILE_FLIPS[flip_index + by_three + 2];
        Tile8 {
            index,
            flip_x,
            flip_y,
            rotate,
        }
    };

    [
        build_tile_8(0),
        build_tile_8(1),
        build_tile_8(2),
        build_tile_8(3),
    ]
}

pub fn enemy_tile16_list() -> Vec<Tile16> {
    (0..ENEMY_TILE_BIT_TABLE.len())
        .map(build_enemy_tile)
        .collect()
}

pub fn sprite_tile16_list() -> Vec<Tile16> {
    (0..SPRITE_TILE_BIT_TABLE.len())
        .map(build_sprite_tile)
        .collect()
}

pub fn map_tile16_list() -> Vec<Tile16> {
    let mut tile16_list: Vec<Tile16> = TILE_16S
        .iter()
        .map(|&tile8s| {
            [
                Tile8::from(tile8s[0]),
                Tile8::from(tile8s[1]),
                Tile8::from(tile8s[2]),
                Tile8::from(tile8s[3]),
            ]
        })
        .collect::<Vec<_>>();
    tile16_list[3] = [
        Tile8::from(239),
        Tile8 {
            index: 239,
            flip_x: true,
            ..Tile8::default()
        },
        Tile8 {
            index: 239,
            flip_y: true,
            ..Tile8::default()
        },
        Tile8 {
            index: 239,
            flip_x: true,
            flip_y: true,
            ..Tile8::default()
        },
    ];
    tile16_list[6] = [
        Tile8::from(92),
        Tile8 {
            index: 92,
            flip_x: true,
            ..Tile8::default()
        },
        Tile8 {
            index: 92,
            flip_y: true,
            ..Tile8::default()
        },
        Tile8 {
            index: 92,
            flip_x: true,
            flip_y: true,
            ..Tile8::default()
        },
    ];
    tile16_list[26] = [
        Tile8 {
            index: 81,
            flip_x: true,
            flip_y: true,
            rotate: true,
        },
        Tile8 {
            index: 65,
            flip_x: true,
            flip_y: true,
            rotate: true,
        },
        Tile8 {
            index: 80,
            flip_x: true,
            flip_y: true,
            rotate: true,
        },
        Tile8 {
            index: 64,
            flip_x: true,
            flip_y: true,
            rotate: true,
        },
    ];
    tile16_list[28] = [
        Tile8::from(93),
        Tile8 {
            index: 93,
            flip_x: true,
            ..Tile8::default()
        },
        Tile8 {
            index: 93,
            flip_y: true,
            ..Tile8::default()
        },
        Tile8 {
            index: 93,
            flip_x: true,
            flip_y: true,
            ..Tile8::default()
        },
    ];
    tile16_list
}

fn decode_graphics(bytes: &[u8]) -> Vec<Tile8Data> {
    bytes
        .chunks(2)
        .collect::<Vec<_>>()
        .chunks(8)
        .map(|tile8| {
            tile8
                .iter()
                .map(|col| {
                    (0..8)
                        .map(|index| {
                            u8::from(col[0] & (1 << (7 - index)) != 0)
                                + u8::from(col[1] & (1 << (7 - index)) != 0) * 2
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

pub fn encode_graphics(tile8_list: Vec<Tile8Data>) -> Vec<u8> {
    tile8_list
        .into_iter()
        .flat_map(|tile8| {
            tile8.into_iter().flat_map(|col| {
                [
                    (0..8).fold(0, |acc, index| acc | ((col[index] & 1) << (7 - index))),
                    (0..8).fold(0, |acc, index| acc | ((col[index] & 2) >> 1 << (7 - index))),
                ]
            })
        })
        .collect()
}
