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

        let colors = if matches!(map, map::SMUGGLERS_ROAD | map::SMUGGLERS_RUIN) {
            PALETTES[index + 75]
        } else {
            PALETTES[MAP_PALETTES[map as usize] as usize * 5 + index]
        };

        lookup_palette(colors)
    } else {
        DEFAULT_PALETTE
    }
}

pub fn get_sprite_palette(index: usize) -> [image::Rgba<u8>; 4] {
    let index = SPRITE_PALETTE_TABLE[index] as usize;
    let colors = SPRITE_PALETTES[index];

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
