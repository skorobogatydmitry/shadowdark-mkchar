//! parse command line args

use clap::{Parser, ValueEnum};
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;

use crate::{
    alignment::Alignment,
    ancestry::{Ancestry, Language},
    class::Class,
    stats::Stats,
};

/// Shadowdark quick characters generator with custom translation support
#[derive(Debug, Parser)]
pub struct Args {
    /// character's class
    #[arg(short, long, value_enum, default_value_t = ClassArg::Suggest)]
    pub class: ClassArg,

    /// character's ancestry, random by default
    #[arg(short, long, value_enum)]
    pub ancestry: Option<Ancestry>,

    /// language for human's ancestry, a random comon language will be chosen if omit
    #[arg(short, long, value_enum)]
    pub language: Option<Language>,

    /// character's alignment, a random alignment will be chosen if omit
    #[arg(long, value_enum)]
    pub alignment: Option<Alignment>,

    /// character's name, a random name will be chosen if omit, accodring to ancestry
    #[arg(short, long, value_enum)]
    pub name: Option<String>,
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
