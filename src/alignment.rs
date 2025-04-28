use std::fmt::Display;

use clap::ValueEnum;
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::translation::LANG_PACK;

#[derive(Debug, EnumIter, PartialEq, Eq, ValueEnum, Clone)]
pub enum Alignment {
    Chaotic,
    Neutral,
    Lawful,
}

impl Alignment {
    pub fn roll() -> Self {
        Self::iter().choose(&mut rand::rng()).unwrap()
    }

    pub fn name(&self) -> String {
        match self {
            Self::Chaotic => LANG_PACK.alignments.chaotic.clone(),
            Self::Neutral => LANG_PACK.alignments.neutral.clone(),
            Self::Lawful => LANG_PACK.alignments.lawful.clone(),
        }
    }
}

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", LANG_PACK.alignment, self.name())
    }
}
