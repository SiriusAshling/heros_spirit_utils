use std::ops::RangeInclusive;

pub enum SpriteType {
    ThreeBytes(ThreeByteSpriteType),
    Other(usize),
}

pub enum ThreeByteSpriteType {
    WindRoute,
}

#[derive(Clone)]
pub enum Sprite {
    WindRoute,
}

const THREE_BYTE_ID_RANGES: [RangeInclusive<u8>; 8] = [1..=13, 16..=24, 42..=103, 107..=143, 148..=189, 191..=192, 194..=213, 240..=250];
const FOUR_BYTE_IDS: [u8; 1] = [106];
const FIVE_BYTE_IDS: [u8; 1] = [15];
const FIVE_BYTE_ID_RANGES: [RangeInclusive<u8>; 1] = [104..=105];
const SEVEN_BYTE_IDS: [u8; 5] = [0, 14, 147, 190, 193];
const SEVEN_BYTE_ID_RANGES: [RangeInclusive<u8>; 1] = [26..=41];

impl From<u8> for SpriteType {
    fn from(id: u8) -> Self {
        match id {
            25 => SpriteType::ThreeBytes(ThreeByteSpriteType::WindRoute),
            _ =>
                if THREE_BYTE_ID_RANGES.iter().any(|range| range.contains(&id)) { SpriteType::Other(3) }
                else if FOUR_BYTE_IDS.contains(&id) { SpriteType::Other(4) }
                else if FIVE_BYTE_IDS.contains(&id) { SpriteType::Other(5) }
                else if FIVE_BYTE_ID_RANGES.iter().any(|range| range.contains(&id)) { SpriteType::Other(5) }
                else if SEVEN_BYTE_IDS.contains(&id) { SpriteType::Other(7) }
                else if SEVEN_BYTE_ID_RANGES.iter().any(|range| range.contains(&id)) { SpriteType::Other(7) }
                else { SpriteType::Other(usize::MAX) }
        }
    }
}
