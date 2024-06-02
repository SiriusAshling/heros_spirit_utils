use crate::data::{
    COLOR_TABLE, ENEMY_PALETTES, MAP_PALETTES, MAP_PALETTE_TABLE, PALETTES, SPRITE_PALETTES,
    SPRITE_PALETTE_TABLE,
};
use crate::map;

pub const DEFAULT_PALETTE: [image::Rgba<u8>; 4] = [
    image::Rgba([255, 255, 255, 255]),
    image::Rgba([170, 170, 170, 255]),
    image::Rgba([85, 85, 85, 255]),
    image::Rgba([0, 0, 0, 255]),
];

fn lookup(color: u8) -> image::Rgba<u8> {
    image::Rgba(COLOR_TABLE[color as usize])
}
pub fn lookup_palette(colors: [u8; 4]) -> [image::Rgba<u8>; 4] {
    [
        lookup(colors[0]),
        lookup(colors[1]),
        lookup(colors[2]),
        lookup(colors[3]),
    ]
}

pub fn get_map_palette(index: usize, map: u8) -> [image::Rgba<u8>; 4] {
    if let Some(index) = MAP_PALETTE_TABLE.get(index) {
        let index = *index as usize;

        let colors = get_map_palette_colors(index, map);

        lookup_palette(colors)
    } else {
        DEFAULT_PALETTE
    }
}

fn get_map_palette_colors(index: usize, map: u8) -> [u8; 4] {
    let map_offset = match map {
        map::GLITCH => return get_glitch_palette_colors(index),
        map::FALLEN_WORLD => 29,
        map::MOONWELL => 31,
        map::HHM_CASTLE_GROUNDS => 1,
        map::HHM_CASTLE_MONILLUD => 23,
        map::HHM_STRANGE_AREA => 22,
        map::HHM_THE_UNDERWORLD => MAP_PALETTES[39] as usize,
        map::HHM_THRONE_ROOM => 36,
        map::SMUGGLERS_ROAD | map::SMUGGLERS_RUIN => 15,
        _ => MAP_PALETTES[map as usize] as usize,
    };

    PALETTES[map_offset * 5 + index]
}

fn get_glitch_palette_colors(index: usize) -> [u8; 4] {
    const GLITCH_FRAME_COUNT: u8 = u8::MAX;

    match index {
        0 => [
            13,
            GLITCH_FRAME_COUNT % 63,
            GLITCH_FRAME_COUNT % 9,
            GLITCH_FRAME_COUNT % 2,
        ],
        1 => [
            GLITCH_FRAME_COUNT % 2,
            GLITCH_FRAME_COUNT % 64,
            GLITCH_FRAME_COUNT % 12,
            GLITCH_FRAME_COUNT % 21,
        ],
        2 => [
            GLITCH_FRAME_COUNT % 3,
            GLITCH_FRAME_COUNT % 19,
            GLITCH_FRAME_COUNT % 23,
            GLITCH_FRAME_COUNT % 12,
        ],
        3 => [
            GLITCH_FRAME_COUNT % 4,
            GLITCH_FRAME_COUNT % 40,
            GLITCH_FRAME_COUNT % 34,
            GLITCH_FRAME_COUNT % 10,
        ],
        4 => [
            GLITCH_FRAME_COUNT % 5,
            GLITCH_FRAME_COUNT % 41,
            GLITCH_FRAME_COUNT % 63,
            GLITCH_FRAME_COUNT % 9,
        ],
        _ => unreachable!(),
    }
}

pub fn get_sprite_palette(index: usize, map_id: u8) -> [image::Rgba<u8>; 4] {
    let colors = if matches!(index, 93 | 94) && matches!(map_id, 33 | 34 | 36) {
        [13, 32, 32, 32]
    } else {
        let index = SPRITE_PALETTE_TABLE[index] as usize;
        SPRITE_PALETTES[index]
    };

    lookup_palette(colors)
}

pub fn get_enemy_palette(index: usize) -> [image::Rgba<u8>; 4] {
    let colors = ENEMY_PALETTES[index * 2];

    lookup_palette(colors)
}

pub fn get_map_sprite_palette(tile: u8, index: usize) -> [image::Rgba<u8>; 4] {
    let colors = PALETTES[(index % 29) * 5 + tile as usize];

    lookup_palette(colors)
}
