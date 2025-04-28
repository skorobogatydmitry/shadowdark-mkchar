use std::fmt::Display;

use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::translation::LANG_PACK;

#[derive(Debug, EnumIter)]
pub enum Background {
    Urchin,
    Wanted,
    CultInitiate,
    ThievesGuild,
    Banished,
    Orphaned,
    WizardsApprentice,
    Jeweler,
    Herbalist,
    Barbarian,
    Mercenary,
    Sailor,
    Alcolyte,
    Soldier,
    Ranger,
    Scout,
    Minstrel,
    Scholar,
    Nobel,
    Chirurgeon,
}

impl Background {
    pub fn roll() -> Self {
        Self::iter().choose(&mut rand::rng()).unwrap()
    }

    pub fn name(&self) -> String {
        match self {
            Self::Urchin => LANG_PACK.backgrounds.urchin.clone(),
            Self::Wanted => LANG_PACK.backgrounds.wanted.clone(),
            Self::CultInitiate => LANG_PACK.backgrounds.cult_initiate.clone(),
            Self::ThievesGuild => LANG_PACK.backgrounds.thieves_guild.clone(),
            Self::Banished => LANG_PACK.backgrounds.banished.clone(),
            Self::Orphaned => LANG_PACK.backgrounds.orphaned.clone(),
            Self::WizardsApprentice => LANG_PACK.backgrounds.wizards_apprentice.clone(),
            Self::Jeweler => LANG_PACK.backgrounds.jeweler.clone(),
            Self::Herbalist => LANG_PACK.backgrounds.herbalist.clone(),
            Self::Barbarian => LANG_PACK.backgrounds.barbarian.clone(),
            Self::Mercenary => LANG_PACK.backgrounds.mercenary.clone(),
            Self::Sailor => LANG_PACK.backgrounds.sailor.clone(),
            Self::Alcolyte => LANG_PACK.backgrounds.alcolyte.clone(),
            Self::Soldier => LANG_PACK.backgrounds.soldier.clone(),
            Self::Ranger => LANG_PACK.backgrounds.ranger.clone(),
            Self::Scout => LANG_PACK.backgrounds.scout.clone(),
            Self::Minstrel => LANG_PACK.backgrounds.minstrel.clone(),
            Self::Scholar => LANG_PACK.backgrounds.scholar.clone(),
            Self::Nobel => LANG_PACK.backgrounds.nobel.clone(),
            Self::Chirurgeon => LANG_PACK.backgrounds.chirurgeon.clone(),
        }
    }
}

impl Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", LANG_PACK.background, self.name())
    }
}
