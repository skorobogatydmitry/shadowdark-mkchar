use std::fmt::Display;

use clap::ValueEnum;
use strum_macros::EnumIter;

use crate::translation::LANG_PACK;

#[derive(Debug, EnumIter, PartialEq, Eq, ValueEnum, Clone)]
pub enum Alignment {
    Chaotic,
    Neutral,
    Lawful,
}

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            LANG_PACK.alignment,
            match self {
                Self::Chaotic => LANG_PACK.alignments.chaotic.clone(),
                Self::Neutral => LANG_PACK.alignments.neutral.clone(),
                Self::Lawful => LANG_PACK.alignments.lawful.clone(),
            }
        )
    }
}
