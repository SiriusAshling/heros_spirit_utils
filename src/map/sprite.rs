use std::cmp::Ordering;

use std::ops::RangeInclusive;

use crate::Result;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
    UnderworldKey = 30,
    PossumCoin = 34,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Gear {
    Hammer = 11,
    Boots = 12,
    Compass = 13,
    Spectacles = 18,
    LavaCharm = 19,
    WindRing = 21,
    SkeletonKey = 20,
    GemSword = 16,
    GemShield = 23,
    RedSword = 31,
    RedShield = 32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Door {
    Gold,
    Silver,
    Red,
    Green,
    Blue,
    Boulder,
    Teal,
    Purple,
    Underworld,
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
        Some(self.cmp(other))
    }
}
impl Ord for Enemy {
    fn cmp(&self, other: &Enemy) -> Ordering {
        match self.strength().cmp(&other.strength()) {
            Ordering::Equal => (*self as u8).cmp(&(*other as u8)),
            cmp => cmp,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Things {
    Transfer,
    Warp,
    CompassWall,
    NGMountain,
    NGPMountain,
    NGPBoulder,
    NGPWall,
    NGPTransfer,
    UnderworldKeyhole,
}

#[derive(Clone)]
pub struct SpriteData {
    pub kind: u8,
    pub extra_bytes: Vec<u8>,
}

const ONE_BYTE_ID_RANGES: [RangeInclusive<u8>; 10] = [
    1..=13,
    16..=25,
    42..=103,
    107..=143,
    148..=160,
    162..=189,
    191..=192,
    194..=196,
    198..=213,
    240..=250,
];
const TWO_BYTE_IDS: [u8; 2] = [105, 106];
const THREE_BYTE_IDS: [u8; 3] = [15, 104, 161];
const FIVE_BYTE_IDS: [u8; 6] = [0, 14, 147, 190, 193, 197];
const FIVE_BYTE_ID_RANGES: [RangeInclusive<u8>; 1] = [26..=41];

impl SpriteData {
    fn read_step(sprite_data: &[u8], index: &mut usize) -> u8 {
        let read = sprite_data[*index];
        *index += 1;
        read
    }

    pub fn read(sprite_data: &[u8], index: &mut usize) -> Result<(u8, u8, Self)> {
        let kind = Self::read_step(sprite_data, index);
        let x = Self::read_step(sprite_data, index);
        let y = Self::read_step(sprite_data, index);
        let size = Self::size_of_kind(kind);
        if size == 0 {
            Err("Unknown sprite")?;
        }
        let extra_bytes = (1..size)
            .map(|_| Self::read_step(sprite_data, index))
            .collect();
        Ok((x, y, Self { kind, extra_bytes }))
    }

    pub fn size_of_kind(kind: u8) -> usize {
        if ONE_BYTE_ID_RANGES.iter().any(|range| range.contains(&kind)) {
            1
        } else if TWO_BYTE_IDS.contains(&kind) {
            2
        } else if THREE_BYTE_IDS.contains(&kind) {
            3
        } else if FIVE_BYTE_IDS.contains(&kind)
            || FIVE_BYTE_ID_RANGES
                .iter()
                .any(|range| range.contains(&kind))
        {
            5
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sprite {
    Collectible(Collectible),
    Gear(Gear),
    Door(Door),
    Enemy(Enemy),
    WindRoute,
    Save,
    Things(Things),
    Other(
        // This is intentionally only used in the Debug implementation
        #[allow(unused)] u8,
    ),
}

impl Sprite {
    pub fn tile_size(self) -> (u8, u8) {
        match self {
            Self::Enemy(Enemy::GDragon) => (3, 2),
            Self::Enemy(Enemy::Basilisk) => (1, 2),
            _ => (1, 1),
        }
    }
}

impl From<u8> for Sprite {
    fn from(id: u8) -> Self {
        match id {
            0 => Sprite::Things(Things::Transfer),
            1 => Sprite::Door(Door::Gold),
            2 | 53 => Sprite::Collectible(Collectible::GoldKey),
            3 | 24 | 69 | 70 => Sprite::Collectible(Collectible::Sword),
            4 => Sprite::Door(Door::Silver),
            5 | 66 => Sprite::Collectible(Collectible::SilverKey),
            6 | 60 => Sprite::Collectible(Collectible::PortalStone),
            7 => Sprite::Collectible(Collectible::Gem),
            8 | 71 => Sprite::Collectible(Collectible::Treasure),
            9 => Sprite::Gear(Gear::Hammer),
            10 => Sprite::Door(Door::Boulder),
            11 => Sprite::Gear(Gear::Boots),
            12 => Sprite::Gear(Gear::Compass),
            13 => Sprite::Things(Things::CompassWall),
            15 => Sprite::Things(Things::Warp),
            22 => Sprite::Door(Door::Blue),
            23 => Sprite::Collectible(Collectible::BlueKey),
            25 => Sprite::WindRoute,
            44 => Sprite::Save,
            51 => Sprite::Gear(Gear::GemSword),
            52 => Sprite::Gear(Gear::Spectacles),
            54 => Sprite::Door(Door::Green),
            55 => Sprite::Collectible(Collectible::GreenKey),
            56 => Sprite::Collectible(Collectible::GoldSword),
            57 | 58 => Sprite::Collectible(Collectible::Heart),
            59 => Sprite::Gear(Gear::LavaCharm),
            61 => Sprite::Door(Door::Red),
            62 => Sprite::Collectible(Collectible::RedKey),
            63 => Sprite::Collectible(Collectible::ShrineKey),
            64 => Sprite::Collectible(Collectible::Shield),
            65 => Sprite::Gear(Gear::GemShield),
            67 => Sprite::Gear(Gear::SkeletonKey),
            68 => Sprite::Collectible(Collectible::Stopwatch),
            72 => Sprite::Gear(Gear::WindRing),
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
            195 => Sprite::Things(Things::NGPMountain),
            196 => Sprite::Things(Things::NGMountain),
            197 => Sprite::Things(Things::NGPTransfer),
            198 => Sprite::Things(Things::NGPBoulder),
            199 => Sprite::Collectible(Collectible::UnderworldKey),
            200 => Sprite::Things(Things::NGPWall),
            201 => Sprite::Door(Door::Underworld),
            208 => Sprite::Things(Things::UnderworldKeyhole),
            209 => Sprite::Gear(Gear::RedShield),
            210 => Sprite::Gear(Gear::RedSword),
            213 => Sprite::Collectible(Collectible::PossumCoin),
            247 => Sprite::Door(Door::Teal),
            248 => Sprite::Collectible(Collectible::TealKey),
            249 => Sprite::Door(Door::Purple),
            250 => Sprite::Collectible(Collectible::PurpleKey),
            _ => Sprite::Other(id),
        }
    }
}
