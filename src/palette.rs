use crate::{data::{MAP_PALETTE_TABLE, PALETTES, MAP_PALETTES, COLOR_TABLE, SPRITE_PALETTE_TABLE, SPRITE_PALETTES}, map::Map};

pub const DEFAULT_PALETTE: [image::Rgba<u8>; 4] = [
    image::Rgba([255, 255, 255, 255]),
    image::Rgba([170, 170, 170, 255]),
    image::Rgba([85, 85, 85, 255]),
    image::Rgba([0, 0, 0, 255]),
];

#[inline]
fn lookup(color: u8) -> image::Rgba<u8> {
    image::Rgba(COLOR_TABLE[color as usize])
}

pub fn get_map_palette(index: usize, map: Map) -> [image::Rgba<u8>; 4] {
    if let Some(index) = MAP_PALETTE_TABLE.get(index) {
        let index = *index as usize;

        let colors = if matches!(map, Map::SmugglersRoad | Map::SmugglersRuin) {
            PALETTES[index + 75]
        } else {
            PALETTES[MAP_PALETTES[u8::from(map) as usize] as usize * 5 + index]
        };

        [lookup(colors[0]), lookup(colors[1]), lookup(colors[2]), lookup(colors[3])]
    } else { DEFAULT_PALETTE }
}

pub fn get_sprite_palette(index: usize) -> [image::Rgba<u8>; 4] {
    let index = SPRITE_PALETTE_TABLE[index] as usize;
    let colors = SPRITE_PALETTES[index];

    [lookup(colors[0]), lookup(colors[1]), lookup(colors[2]), lookup(colors[3])]
}
