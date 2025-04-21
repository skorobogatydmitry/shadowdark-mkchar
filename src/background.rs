use std::fmt::Display;

use strum_macros::EnumIter;

use crate::langpack;

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

impl Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            langpack::PACK.background,
            match self {
                Self::Urchin => langpack::PACK.backgrounds.urchin.clone(),
                Self::Wanted => langpack::PACK.backgrounds.wanted.clone(),
                Self::CultInitiate => langpack::PACK.backgrounds.cult_initiate.clone(),
                Self::ThievesGuild => langpack::PACK.backgrounds.thieves_guild.clone(),
                Self::Banished => langpack::PACK.backgrounds.banished.clone(),
                Self::Orphaned => langpack::PACK.backgrounds.orphaned.clone(),
                Self::WizardsApprentice => langpack::PACK.backgrounds.wizards_apprentice.clone(),
                Self::Jeweler => langpack::PACK.backgrounds.jeweler.clone(),
                Self::Herbalist => langpack::PACK.backgrounds.herbalist.clone(),
                Self::Barbarian => langpack::PACK.backgrounds.barbarian.clone(),
                Self::Mercenary => langpack::PACK.backgrounds.mercenary.clone(),
                Self::Sailor => langpack::PACK.backgrounds.sailor.clone(),
                Self::Alcolyte => langpack::PACK.backgrounds.alcolyte.clone(),
                Self::Soldier => langpack::PACK.backgrounds.soldier.clone(),
                Self::Ranger => langpack::PACK.backgrounds.ranger.clone(),
                Self::Scout => langpack::PACK.backgrounds.scout.clone(),
                Self::Minstrel => langpack::PACK.backgrounds.minstrel.clone(),
                Self::Scholar => langpack::PACK.backgrounds.scholar.clone(),
                Self::Nobel => langpack::PACK.backgrounds.nobel.clone(),
                Self::Chirurgeon => langpack::PACK.backgrounds.chirurgeon.clone(),
            }
        )
    }
}
