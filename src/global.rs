use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::{FromRepr, VariantArray};

use crate::util;
use crate::Result;

#[derive(Serialize, Deserialize)]
pub struct GlobalSave {
    pub sequence: String,
    pub flags: HashMap<GlobalFlag, bool>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, FromRepr, VariantArray)]
#[repr(u8)]
pub enum GlobalFlag {
    DefeatedConvergence,
    DefeatedGlitch,
    DefeatedWitch,
    ObtainedGemShield,
    ObtainedGemSword,
    ObtainedGreenShield,
    ObtainedGreenSword,
    ObtainedRedShield,
    ObtainedRedSword,
    ObtainedWitchCloakNormal,
    StartedGame,
    StartedNewGamePlus,
    StartedNewGamePlusPlus,
    StartedNewGamePlusPlusPlus,
    UnlockedGreenEquipment,
    TheEye,
    DeprecatedEgg,
    TamedRabbit,
    EvilBunny,
    Cat1,
    Cat2,
    Cat3,
    HerosSprint,
    NewGameMinus,
    Eclipsed,
    HaphyStarted,
    HaphyEnded,
    Horsie,
    Unicorn,
    Dicorn,
    Heartless,
    Royal,
}
impl GlobalFlag {
    fn has(&self, byte: u8) -> bool {
        match self {
            Self::DefeatedConvergence => byte == 160,
            Self::DefeatedGlitch => byte == 255,
            Self::DefeatedWitch => byte == 228,
            Self::ObtainedGemShield => byte == 196,
            Self::ObtainedGemSword => byte == 7,
            Self::ObtainedGreenShield => byte == 180,
            Self::ObtainedGreenSword => byte == 68,
            Self::ObtainedRedShield => byte == 88,
            Self::ObtainedRedSword => byte == 100,
            Self::ObtainedWitchCloakNormal => byte == 128,
            Self::StartedGame => byte == 242,
            Self::StartedNewGamePlus => byte == 9,
            Self::StartedNewGamePlusPlus => byte == 95,
            Self::StartedNewGamePlusPlusPlus => byte == 254,
            Self::UnlockedGreenEquipment => byte == 96,
            Self::TheEye => byte == 150,
            Self::DeprecatedEgg => byte == 81,
            Self::TamedRabbit => byte == 35,
            Self::EvilBunny => byte == 65,
            Self::Cat1 => byte == 3,
            Self::Cat2 => byte == 119,
            Self::Cat3 => byte == 83,
            Self::HerosSprint => byte == 0,
            Self::NewGameMinus => byte == 34,
            Self::Eclipsed => byte == 254,
            Self::HaphyStarted => byte == 170,
            Self::HaphyEnded => byte == 0,
            Self::Horsie => byte == 255,
            Self::Unicorn => byte == 255,
            Self::Dicorn => byte == 255,
            Self::Heartless => byte == 255,
            Self::Royal => byte == 1,
        }
    }
}

pub fn decode() -> Result<()> {
    let data = util::read("global")?;

    let sequence = data[..10].iter().map(|number| number.to_string()).collect();

    let flags = GlobalFlag::VARIANTS
        .iter()
        .copied()
        .map(|flag| (flag, flag.has(data[flag as usize + 10])))
        .collect();

    let global_save = GlobalSave { sequence, flags };
    let out = serde_json::to_string_pretty(&global_save)?;

    util::write("global.json", out)?;

    Ok(())
}
