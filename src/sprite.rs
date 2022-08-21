use std::fmt::{self, Display};
use std::ops::RangeInclusive;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Enemy {
    Guard = 0,
    BSlime = 1,
    Bat = 2,
    YBat = 3,
    Fireball = 4,
    PFireball = 5,
    Gargoyle = 6,
    Ghost = 7,
    SGuard = 8,
    Pirate = 9,
    Rat = 10,
    GRat = 11,
    Skeleton = 12,
    DSkeleton = 13,
    GSkeleton = 14,
    PSlime = 15,
    GSlime = 16,
    RSlime = 17,
    YSlime = 18,
    Spider = 19,
    WSpider = 20,
    Whelp = 21,
    BWhelp = 22,
    GWhelp = 23,
    FairyR = 24,
    FairyG = 25,
    FairyB = 26,
    GDragon = 27,
    // BDragon,
    Basilisk = 29,
    CloneP = 30,
    CloneW = 31,
    // WFireball = 32,
    // Cloneball = 33,
    Witch = 34,
    Rabbit = 39,
    Glitch = 40,
    GWitch = 41,
    // RDragon,
    // Ragnarok = 43,
    // EvilBunny = 44,
    // DarkGhost = 45,
}

impl Enemy {
    pub fn strength(self) -> u8 {
        match self {
            Enemy::Guard => 50,
            Enemy::BSlime => 5,
            Enemy::Bat => 10,
            Enemy::YBat => 55,
            Enemy::Fireball => 100,
            Enemy::PFireball => 100,
            Enemy::Gargoyle => 80,
            Enemy::Ghost => 100,
            Enemy::SGuard => 100,
            Enemy::Pirate => 35,
            Enemy::Rat => 25,
            Enemy::GRat => 70,
            Enemy::Skeleton => 20,
            Enemy::DSkeleton => 45,
            Enemy::GSkeleton => 75,
            Enemy::PSlime => 65,
            Enemy::GSlime => 15,
            Enemy::RSlime => 50,
            Enemy::YSlime => 30,
            Enemy::Spider => 15,
            Enemy::WSpider => 65,
            Enemy::Whelp => 60,
            Enemy::BWhelp => 90,
            Enemy::GWhelp => 40,
            Enemy::FairyR => 95,
            Enemy::FairyG => 95,
            Enemy::FairyB => 95,
            Enemy::GDragon => 75,
            Enemy::Basilisk => 95,
            Enemy::CloneP => 10,
            Enemy::CloneW => 10,
            Enemy::Witch => 0,
            Enemy::Rabbit => 1,
            Enemy::Glitch => 100,
            Enemy::GWitch => 100,
        }
    }
}

impl PartialOrd for Enemy {
    fn partial_cmp(&self, other: &Enemy) -> Option<Ordering> {
        self.strength().partial_cmp(&other.strength())
    }
}
impl Ord for Enemy {
    fn cmp(&self, other: &Enemy) -> Ordering {
        self.strength().cmp(&other.strength())
    }
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

impl Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sprite::Collectible(collectible) => write!(f, "{:?}", collectible),
            Sprite::Door(door) => write!(f, "{:?} Door", door),
            Sprite::Enemy(enemy) => write!(f, "{:?}", enemy),
            Sprite::WindRoute => write!(f, "WindRoute"),
            Sprite::Save => write!(f, "Save"),
            Sprite::Other(usize) => write!(f, "{}", usize),
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
            114 => Sprite::Enemy(Enemy::GDragon),
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
            126 => Sprite::Enemy(Enemy::Skeleton),
            127 | 128 => Sprite::Enemy(Enemy::DSkeleton),
            129 => Sprite::Enemy(Enemy::GSkeleton),
            130 => Sprite::Enemy(Enemy::BSlime),
            131 | 132 => Sprite::Enemy(Enemy::PSlime),
            133 => Sprite::Enemy(Enemy::GSlime),
            134 => Sprite::Enemy(Enemy::RSlime),
            135 => Sprite::Enemy(Enemy::YSlime),
            137 => Sprite::Enemy(Enemy::Spider),
            138 => Sprite::Enemy(Enemy::WSpider),
            139 | 140 => Sprite::Enemy(Enemy::Whelp),
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
