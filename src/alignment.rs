use std::fmt::Display;

use strum_macros::EnumIter;

use crate::langpack;

#[derive(Debug, EnumIter, PartialEq, Eq)]
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
            langpack::PACK.alignment,
            match self {
                Self::Chaotic => langpack::PACK.alignments.chaotic.clone(),
                Self::Neutral => langpack::PACK.alignments.neutral.clone(),
                Self::Lawful => langpack::PACK.alignments.lawful.clone(),
            }
        )
    }
}
