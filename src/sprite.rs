use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
pub enum Collectible {
    GoldKey,
    Sword,
    SilverKey,
    PortalStone,
    Gem,
    Treasure,
    BlueKey,
    GreenKey,
    GoldSword,
    Heart,
    RedKey,
    ShrineKey,
    Shield,
    Stopwatch,
    GemHeart,
    PossumCoin,
    TealKey,
    PurpleKey,
}

impl From<Collectible> for u8 {
    fn from(collectible: Collectible) -> u8 {
        match collectible {
            Collectible::GoldKey => 0,
            Collectible::Sword => 6,
            Collectible::SilverKey => 1,
            Collectible::PortalStone => 8,
            Collectible::Gem => 9,
            Collectible::Treasure => 10,
            Collectible::BlueKey => 5,
            Collectible::GreenKey => 4,
            Collectible::GoldSword => 15,
            Collectible::Heart => 17,
            Collectible::RedKey => 3,
            Collectible::ShrineKey => 14,
            Collectible::Shield => 22,
            Collectible::Stopwatch => 24,
            Collectible::GemHeart => 25,
            Collectible::PossumCoin => 34,
            Collectible::TealKey => 27,
            Collectible::PurpleKey => 28,
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
            2 | 53 => Sprite::Collectible(Collectible::GoldKey),
            3 | 24 | 69 | 70 => Sprite::Collectible(Collectible::Sword),
            5 | 66 => Sprite::Collectible(Collectible::SilverKey),
            6 | 60 => Sprite::Collectible(Collectible::PortalStone),
            7 => Sprite::Collectible(Collectible::Gem),
            8 | 71 => Sprite::Collectible(Collectible::Treasure),
            23 => Sprite::Collectible(Collectible::BlueKey),
            25 => Sprite::WindRoute,
            44 => Sprite::Save,
            55 => Sprite::Collectible(Collectible::GreenKey),
            56 => Sprite::Collectible(Collectible::GoldSword),
            57 | 58 => Sprite::Collectible(Collectible::Heart),
            62 => Sprite::Collectible(Collectible::RedKey),
            63 => Sprite::Collectible(Collectible::ShrineKey),
            64 => Sprite::Collectible(Collectible::Shield),
            68 => Sprite::Collectible(Collectible::Stopwatch),
            152 => Sprite::Collectible(Collectible::GemHeart),
            213 => Sprite::Collectible(Collectible::PossumCoin),
            248 => Sprite::Collectible(Collectible::TealKey),
            250 => Sprite::Collectible(Collectible::PurpleKey),
            _ =>
                if THREE_BYTE_ID_RANGES.iter().any(|range| range.contains(&id)) { Sprite::Other(3) }
                else if FOUR_BYTE_IDS.contains(&id) { Sprite::Other(4) }
                else if FIVE_BYTE_IDS.contains(&id) { Sprite::Other(5) }
                else if SEVEN_BYTE_IDS.contains(&id) || SEVEN_BYTE_ID_RANGES.iter().any(|range| range.contains(&id)) { Sprite::Other(7) }
                else { panic!("Unexpected sprite data") }
        }
    }
}
