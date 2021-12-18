use std::ops::RangeInclusive;

#[derive(Clone)]
pub enum Sprite {
    WindRoute,
    Other(usize),
}

impl Sprite {
    pub fn data_bytes(&self) -> usize {
        match self {
            Sprite::WindRoute => 3,
            Sprite::Other(bytes) => *bytes,
        }
    }
}

const THREE_BYTE_ID_RANGES: [RangeInclusive<u8>; 8] = [1..=13, 16..=24, 42..=103, 107..=143, 148..=189, 191..=192, 194..=213, 240..=250];
const FOUR_BYTE_IDS: [u8; 1] = [106];
const FIVE_BYTE_IDS: [u8; 1] = [15];
const FIVE_BYTE_ID_RANGES: [RangeInclusive<u8>; 1] = [104..=105];
const SEVEN_BYTE_IDS: [u8; 5] = [0, 14, 147, 190, 193];
const SEVEN_BYTE_ID_RANGES: [RangeInclusive<u8>; 1] = [26..=41];

impl From<u8> for Sprite {
    fn from(id: u8) -> Self {
        match id {
            25 => Sprite::WindRoute,
            _ =>
                if THREE_BYTE_ID_RANGES.iter().any(|range| range.contains(&id)) { Sprite::Other(3) }
                else if FOUR_BYTE_IDS.contains(&id) { Sprite::Other(4) }
                else if FIVE_BYTE_IDS.contains(&id) || FIVE_BYTE_ID_RANGES.iter().any(|range| range.contains(&id)) { Sprite::Other(5) }
                else if SEVEN_BYTE_IDS.contains(&id) || SEVEN_BYTE_ID_RANGES.iter().any(|range| range.contains(&id)) { Sprite::Other(7) }
                else { Sprite::Other(1) }
        }
    }
}
