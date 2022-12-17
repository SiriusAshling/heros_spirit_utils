use crate::data::{TILE_16S, SPRITE_TILE_BIT_TABLE, SPRITE_TILE_FLIP_TABLE, SPRITE_TILE_BITS, SPRITE_TILE_FLIPS, ENEMY_TILE_BIT_TABLE, ENEMY_TILE_BITS, ENEMY_TILE_FLIPS, MAP_TILE_BITS};

pub struct TileData {
    pub tile8_list: Vec<Tile8Data>,
    pub map_tile16_list: Vec<Tile16>,
    pub sprite_tile16_list: Vec<Tile16>,
    pub enemy_tile16_list: Vec<Tile16>,
    pub map_sprite_tile16_list: Vec<Tile16>,
}

pub type Tile8Data = Vec<Vec<u8>>;

impl From<Vec<Tile8Data>> for TileData {
    fn from(tile8_list: Vec<Tile8Data>) -> Self {
        Self {
            tile8_list,
            map_tile16_list: map_tile16_list(),
            sprite_tile16_list: sprite_tile16_list(),
            enemy_tile16_list: enemy_tile16_list(),
            map_sprite_tile16_list: map_sprite_tile16_list(),
        }
    }
}

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

fn build_enemy_tile(id: usize) -> Tile16 {
    let bit_index = ENEMY_TILE_BIT_TABLE[id] as usize * 8;
    let flip_index = bit_index * 3;

    let build_tile_8 = |tile_index| {
        let by_three = tile_index * 3;

        let index = ENEMY_TILE_BITS[bit_index + tile_index];
        let flipx = ENEMY_TILE_FLIPS[flip_index + by_three];
        let flipy = ENEMY_TILE_FLIPS[flip_index + by_three + 1];
        let rotate = ENEMY_TILE_FLIPS[flip_index + by_three + 2];
        Tile8 { index, flipx, flipy, rotate }
    };

    [build_tile_8(0), build_tile_8(1), build_tile_8(2), build_tile_8(3)]
}

fn build_sprite_tile(id: usize) -> Tile16 {
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

fn build_map_sprite_tile(index: usize) -> Tile16 {
    let id = index / 29;
    let bit_index = id * 4;
    let flip_index = if id == 2 { 60 } else { 0 };

    let build_tile_8 = |tile_index| {
        let by_three = tile_index * 3;

        let index = MAP_TILE_BITS[bit_index + tile_index];
        let flipx = SPRITE_TILE_FLIPS[flip_index + by_three];
        let flipy = SPRITE_TILE_FLIPS[flip_index + by_three + 1];
        let rotate = SPRITE_TILE_FLIPS[flip_index + by_three + 2];
        Tile8 { index, flipx, flipy, rotate }
    };

    [build_tile_8(0), build_tile_8(1), build_tile_8(2), build_tile_8(3)]
}

pub fn enemy_tile16_list() -> Vec<Tile16> {
    (0..ENEMY_TILE_BIT_TABLE.len()).map(build_enemy_tile).collect()
}

pub fn sprite_tile16_list() -> Vec<Tile16> {
    (0..SPRITE_TILE_BIT_TABLE.len()).map(build_sprite_tile).collect()
}

pub fn map_sprite_tile16_list() -> Vec<Tile16> {
    (0..116).map(build_map_sprite_tile).collect()
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

pub fn decode_graphics(bytes: Vec<u8>) -> Vec<Tile8Data> {
    bytes.chunks(2).collect::<Vec<_>>().chunks(8).map(|tile8|
        tile8.iter().map(|col|
            (0..8).map(|index|
                (col[0] & 1 << (7 - index) != 0) as u8 +
                (col[1] & 1 << (7 - index) != 0) as u8 * 2
            ).collect()
        ).collect()
    ).collect()
}

pub fn encode_graphics(tile8_list: Vec<Tile8Data>) -> Vec<u8> {
    tile8_list.into_iter().flat_map(|tile8|
        tile8.into_iter().flat_map(|col|
            [
                (0..8).fold(0, |acc, index| acc | ((col[index] & 1) << (7 - index))),
                (0..8).fold(0, |acc, index| acc | ((col[index] & 2) >> 1 << (7 - index)))
            ]
        )
    ).collect()
}
