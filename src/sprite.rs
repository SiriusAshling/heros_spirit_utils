use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
pub enum Collectible {
    GoldKey,
    Sword,
}

impl From<Collectible> for u8 {
    fn from(collectible: Collectible) -> u8 {
        match collectible {
            Collectible::GoldKey => 0,
            Collectible::Sword => 6,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Sprite {
    Collectible(Collectible),
    WindRoute,
    Save,
    Other(usize),
}

impl Sprite {
    pub fn data_bytes(&self) -> usize {
        match self {
            Sprite::Collectible(_) | Sprite::WindRoute | Sprite::Save => 3,
            Sprite::Other(bytes) => *bytes,
        }
    }
}

const THREE_BYTE_ID_RANGES: [RangeInclusive<u8>; 10] = [1..=13, 16..=25, 42..=103, 107..=143, 148..=160, 162..=189, 191..=192, 194..=196, 198..=213, 240..=250];
const FOUR_BYTE_IDS: [u8; 2] = [105, 106];
const FIVE_BYTE_IDS: [u8; 3] = [15, 104, 161];
const SEVEN_BYTE_IDS: [u8; 6] = [0, 14, 147, 190, 193, 197];
const SEVEN_BYTE_ID_RANGES: [RangeInclusive<u8>; 1] = [26..=41];

impl From<u8> for Sprite {
    fn from(id: u8) -> Self {
        match id {
            2 => Sprite::Collectible(Collectible::GoldKey),
            3 => Sprite::Collectible(Collectible::Sword),
            25 => Sprite::WindRoute,
            44 => Sprite::Save,
            _ =>
                if THREE_BYTE_ID_RANGES.iter().any(|range| range.contains(&id)) { Sprite::Other(3) }
                else if FOUR_BYTE_IDS.contains(&id) { Sprite::Other(4) }
                else if FIVE_BYTE_IDS.contains(&id) { Sprite::Other(5) }
                else if SEVEN_BYTE_IDS.contains(&id) || SEVEN_BYTE_ID_RANGES.iter().any(|range| range.contains(&id)) { Sprite::Other(7) }
                else { panic!("Unexpected sprite data") }
        }
    }
}
