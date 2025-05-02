use std::fmt::Display;

use clap::ValueEnum;
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{alignment::Alignment, translation::LANG_PACK};

#[derive(Debug, EnumIter, Clone, ValueEnum)]
pub enum Deity {
    SaintTerragnis,
    Gede,
    MadeeraTheCovenant,
    Ord,
    Memnon,
    Ramlaat,
    ShuneTheVile,
    TheLost,
}

impl Deity {
    pub fn alignment(&self) -> Option<Alignment> {
        match self {
            Self::SaintTerragnis => Some(Alignment::Lawful),
            Self::Gede => Some(Alignment::Neutral),
            Self::MadeeraTheCovenant => Some(Alignment::Lawful),
            Self::Ord => Some(Alignment::Neutral),
            Self::Memnon => Some(Alignment::Chaotic),
            Self::Ramlaat => Some(Alignment::Chaotic),
            Self::ShuneTheVile => Some(Alignment::Chaotic),
            Self::TheLost => None,
        }
    }

    pub fn roll() -> Self {
        Self::iter().choose(&mut rand::rng()).unwrap()
    }

    pub fn name(&self) -> String {
        match self {
            Self::SaintTerragnis => LANG_PACK.deities.saint_terragnis.clone(),
            Self::Gede => LANG_PACK.deities.gede.clone(),
            Self::MadeeraTheCovenant => LANG_PACK.deities.madeera_the_covenant.clone(),
            Self::Ord => LANG_PACK.deities.ord.clone(),
            Self::Memnon => LANG_PACK.deities.memnon.clone(),
            Self::Ramlaat => LANG_PACK.deities.ramlaat.clone(),
            Self::ShuneTheVile => LANG_PACK.deities.shune_the_vile.clone(),
            Self::TheLost => LANG_PACK.deities.the_lost.clone(),
        }
    }

    pub fn by_alignment(alignment: &Alignment) -> Deity {
        Self::iter()
            .filter(|d| d.alignment().is_some_and(|a| a == *alignment))
            .choose(&mut rand::rng())
            .unwrap()
    }
}

impl Display for Deity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", LANG_PACK.deity, self.name())?;
        if let Some(a) = self.alignment() {
            write!(f, " ({})", a)?;
        }
        return Ok(());
    }
}
