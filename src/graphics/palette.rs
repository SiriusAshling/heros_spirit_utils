use crate::data::{
    COLOR_TABLE, ENEMY_PALETTES, MAP_PALETTE_TABLE, SPRITE_PALETTES, SPRITE_PALETTE_TABLE,
};
use crate::graphics::DrawData;
use crate::map::Map;

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

impl DrawData<'_> {
    pub fn get_map_palette(&self, index: usize, map: u8) -> [image::Rgba<u8>; 4] {
        if let Some(index) = MAP_PALETTE_TABLE.get(index) {
            let index = *index as usize;

            let colors = self.get_map_palette_colors(index, map);

            lookup_palette(colors)
        } else {
            DEFAULT_PALETTE
        }
    }

    fn get_map_palette_colors(&self, index: usize, map: u8) -> [u8; 4] {
        if map == Map::GLITCH {
            get_glitch_palette_colors(index)
        } else {
            let palette_index = self.get_palette_index(map);
            self.map_colors.map_colors[palette_index % self.map_colors.map_colors.len()][index]
        }
    }

    fn get_palette_index(&self, map: u8) -> usize {
        // TODO variants
        self.map_meta[&(map as usize)].colors
    }
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
