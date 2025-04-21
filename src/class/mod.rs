use std::fmt::Display;

use strum_macros::EnumIter;

use crate::langpack;

#[derive(Debug, Clone, EnumIter)]
pub enum Class {
    Fighter,
    Thief,
    Wizard,
    Priest,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fighter => langpack::PACK.class_args.fighter.clone(),
                Self::Thief => langpack::PACK.class_args.thief.clone(),
                Self::Wizard => langpack::PACK.class_args.wizard.clone(),
                Self::Priest => langpack::PACK.class_args.priest.clone(),
            }
        )
    }
}
