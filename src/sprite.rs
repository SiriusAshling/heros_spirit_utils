use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
pub enum Collectible {
    GoldKey,
    SilverKey,
    RedKey = 3,
    GreenKey,
    BlueKey,
    Sword,
    PortalStone = 8,
    Gem,
    Treasure,
    ShrineKey = 14,
    GoldSword,
    Heart = 17,
    Shield = 22,
    Stopwatch = 24,
    GemHeart,
    TealKey = 27,
    PurpleKey = 28,
    PossumCoin = 34,
}

#[derive(Debug, Clone, Copy)]
pub enum Door {
    Gold,
    Silver,
    Red,
    Green,
    Blue,
    Boulder,
    Teal,
    Purple,
}

#[derive(Debug, Clone, Copy)]
pub enum Enemy {
    Guard = 1,
    BSlime = 2,
    Bat = 3,
    YBat = 4,
    Fireball = 5,
    PFireball = 6,
    Gargoyle = 7,
    Ghost = 8,
    SGuard = 9,
    Pirate = 10,
    Rat = 11,
    GRat = 12,
    Skeleton = 13,
    DSkeleton = 14,
    GSkeleton = 15,
    PSlime = 16,
    GSlime = 17,
    RSlime = 18,
    YSlime = 19,
    Spider = 20,
    WSpider = 21,
    Whelp = 22,
    BWhelp = 23,
    GWhelp = 24,
    FairyR = 25,
    FairyG = 26,
    FairyB = 27,
    // GDragon,
    Dragon = 28,
    // BDragon,
    Basilisk = 33,
    CloneP = 34,
    CloneW = 35,
    // WFireball = 36,
    // Cloneball = 37,
    Witch = 38,
    Rabbit = 39,
    Glitch = 40,
    GWitch = 41,
    // RDragon,
    // Ragnarok = 43,
    // EvilBunny = 44,
    // DarkGhost = 45,
}

#[derive(Debug, Clone)]
pub enum Sprite {
    Collectible(Collectible),
    Door(Door),
    Enemy(Enemy),
    WindRoute,
    Save,
    Other(usize),
}

impl Sprite {
    pub fn data_bytes(&self) -> usize {
        match self {
            Sprite::Collectible(_) | Sprite::Door(_) | Sprite::Enemy(_) | Sprite::WindRoute | Sprite::Save => 3,
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
            1 => Sprite::Door(Door::Gold),
            2 | 53 => Sprite::Collectible(Collectible::GoldKey),
            3 | 24 | 69 | 70 => Sprite::Collectible(Collectible::Sword),
            4 => Sprite::Door(Door::Silver),
            5 | 66 => Sprite::Collectible(Collectible::SilverKey),
            6 | 60 => Sprite::Collectible(Collectible::PortalStone),
            7 => Sprite::Collectible(Collectible::Gem),
            8 | 71 => Sprite::Collectible(Collectible::Treasure),
            10 => Sprite::Door(Door::Boulder),
            22 => Sprite::Door(Door::Blue),
            23 => Sprite::Collectible(Collectible::BlueKey),
            25 => Sprite::WindRoute,
            44 => Sprite::Save,
            54 => Sprite::Door(Door::Green),
            55 => Sprite::Collectible(Collectible::GreenKey),
            56 => Sprite::Collectible(Collectible::GoldSword),
            57 | 58 => Sprite::Collectible(Collectible::Heart),
            61 => Sprite::Door(Door::Red),
            62 => Sprite::Collectible(Collectible::RedKey),
            63 => Sprite::Collectible(Collectible::ShrineKey),
            64 => Sprite::Collectible(Collectible::Shield),
            68 => Sprite::Collectible(Collectible::Stopwatch),
            108 => Sprite::Enemy(Enemy::Basilisk),
            109 => Sprite::Enemy(Enemy::Bat),
            110 | 111 => Sprite::Enemy(Enemy::YBat),
            112 => Sprite::Enemy(Enemy::CloneW),
            113 => Sprite::Enemy(Enemy::CloneP),
            114 => Sprite::Enemy(Enemy::Dragon),
            115 => Sprite::Enemy(Enemy::Fireball),
            116 => Sprite::Enemy(Enemy::PFireball),
            117 => Sprite::Enemy(Enemy::Gargoyle),
            118 => Sprite::Enemy(Enemy::Ghost),
            119 => Sprite::Enemy(Enemy::Guard),
            120 => Sprite::Enemy(Enemy::SGuard),
            121 => Sprite::Enemy(Enemy::Glitch),
            122 => Sprite::Enemy(Enemy::Pirate),
            124 => Sprite::Enemy(Enemy::Rat),
            125 => Sprite::Enemy(Enemy::GRat),
            126 | 127 => Sprite::Enemy(Enemy::Skeleton),
            128 => Sprite::Enemy(Enemy::DSkeleton),
            129 => Sprite::Enemy(Enemy::GSkeleton),
            130 | 131 => Sprite::Enemy(Enemy::BSlime),
            132 => Sprite::Enemy(Enemy::PSlime),
            133 => Sprite::Enemy(Enemy::GSlime),
            134 => Sprite::Enemy(Enemy::RSlime),
            135 => Sprite::Enemy(Enemy::YSlime),
            137 => Sprite::Enemy(Enemy::Spider),
            138 | 139 => Sprite::Enemy(Enemy::WSpider),
            140 => Sprite::Enemy(Enemy::Whelp),
            141 => Sprite::Enemy(Enemy::BWhelp),
            142 => Sprite::Enemy(Enemy::GWhelp),
            143 => Sprite::Enemy(Enemy::Witch),
            148 => Sprite::Enemy(Enemy::FairyR),
            149 => Sprite::Enemy(Enemy::FairyG),
            150 => Sprite::Enemy(Enemy::FairyB),
            152 => Sprite::Collectible(Collectible::GemHeart),
            160 => Sprite::Enemy(Enemy::Rabbit),
            191 => Sprite::Enemy(Enemy::GWitch),
            213 => Sprite::Collectible(Collectible::PossumCoin),
            247 => Sprite::Door(Door::Teal),
            248 => Sprite::Collectible(Collectible::TealKey),
            249 => Sprite::Door(Door::Purple),
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
