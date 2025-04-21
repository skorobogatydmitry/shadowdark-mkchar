use std::fmt::Display;

use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{alignment::Alignment, langpack};

#[derive(Debug, EnumIter)]
pub enum Deities {
    SaintTerragnis,
    Gede,
    MadeeraTheCovenant,
    Ord,
    Memnon,
    Ramlaat,
    ShuneTheVile,
    TheLost,
}

impl Deities {
    fn alignment(&self) -> Option<Alignment> {
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
}

pub struct Deity {
    pub name: String,
    pub kind: Deities,
}

impl Deity {
    pub fn roll(alignment: &Alignment) -> Self {
        let mut kind = Deities::iter().choose(&mut rand::rng()).unwrap();
        while kind.alignment().as_ref().unwrap_or(&alignment) != alignment {
            // TODO: non-infinite
            kind = Deities::iter().choose(&mut rand::rng()).unwrap();
        }
        Self {
            name: match kind {
                Deities::SaintTerragnis => langpack::PACK.deities.saint_terragnis.clone(),
                Deities::Gede => langpack::PACK.deities.gede.clone(),
                Deities::MadeeraTheCovenant => langpack::PACK.deities.madeera_the_covenant.clone(),
                Deities::Ord => langpack::PACK.deities.ord.clone(),
                Deities::Memnon => langpack::PACK.deities.memnon.clone(),
                Deities::Ramlaat => langpack::PACK.deities.ramlaat.clone(),
                Deities::ShuneTheVile => langpack::PACK.deities.shune_the_vile.clone(),
                Deities::TheLost => langpack::PACK.deities.the_lost.clone(),
            },
            kind,
        }
    }
}

impl Display for Deity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", langpack::PACK.deity, self.name)?;
        if let Some(a) = self.kind.alignment() {
            write!(f, " ({})", a)?;
        }
        return Ok(());
    }
}
