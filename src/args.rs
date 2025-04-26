//! parse command line args

use clap::{Parser, ValueEnum};
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;

use crate::{ancestry::Ancestry, class::Class, stats::Stats};

/// Shadowdark quick characters generator with custom translation support
#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, value_enum, default_value_t = ClassArg::Suggest)]
    pub class: ClassArg,
    #[arg(short, long, value_enum)]
    /// ancestry of the character, random by default
    pub ancestry: Option<Ancestry>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ClassArg {
    /// make a zero-level character
    Zero,
    /// use random class for a 1st-level character
    Random,
    /// use stats to choose the most suitable class
    Suggest,
    Fighter,
    Thief,
    Wizard,
    Priest,
}

impl ClassArg {
    pub fn choose(self, stats: &Stats) -> Option<Class> {
        match self {
            Self::Zero => None,
            Self::Fighter => Some(Class::Fighter),
            Self::Thief => Some(Class::Thief),
            Self::Wizard => Some(Class::Wizard),
            Self::Priest => Some(Class::Priest),
            Self::Suggest => Some(stats.suggest_class()),
            Self::Random => Some(Class::iter().choose(&mut rand::rng()).unwrap()),
        }
    }
}
