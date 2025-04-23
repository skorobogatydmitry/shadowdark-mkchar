use std::fmt::Display;

use rand::prelude::*;
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::translation::LANG_PACK;

#[derive(Debug, EnumIter, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Ancestry {
    Dwarf(String),
    Kobold(String),
    Elf(String),
    Goblin(String),
    Halfling(String),
    HalfOrc(String),
    Human(String),
}

impl Ancestry {
    pub fn roll() -> Self {
        if Self::iter().len() != LANG_PACK.ancestries.len() {
            panic!("{}", LANG_PACK.error_messages.not_all_ancestries);
        }
        LANG_PACK
            .ancestries
            .choose(&mut rand::rng())
            .unwrap()
            .clone()
    }
    pub fn languages(&self) -> Vec<Language> {
        match self {
            Self::Dwarf(_) => vec![Language::Common, Language::Dwarwish],
            Self::Kobold(_) => vec![Language::Common, Language::Draconic],
            Self::Elf(_) => vec![Language::Common, Language::Elvish, Language::Sylvan],
            Self::Goblin(_) => vec![Language::Common, Language::Goblin],
            Self::Halfling(_) => vec![Language::Common],
            Self::HalfOrc(_) => vec![Language::Common, Language::Orchish],
            Self::Human(_) => vec![
                Language::Common,
                Language::AnyOf(
                    Language::iter()
                        .filter_map(|l| {
                            if l.is_common() && l != Language::Common {
                                Some(l)
                            } else {
                                None
                            }
                        })
                        .collect(),
                ),
            ],
        }
    }

    fn feature(&self) -> AncestryFeature {
        match self {
            Self::Dwarf(_) => AncestryFeature::Stout,
            Self::Kobold(_) => AncestryFeature::Knack,
            Self::Elf(_) => AncestryFeature::Farsight,
            Self::Goblin(_) => AncestryFeature::KeenSenses,
            Self::Halfling(_) => AncestryFeature::Stealthy,
            Self::HalfOrc(_) => AncestryFeature::Mighty,
            Self::Human(_) => AncestryFeature::Ambitious,
        }
    }
}

impl Display for Ancestry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} | {}",
            LANG_PACK.ancestry,
            match self {
                Self::Dwarf(val) => val,
                Self::Elf(val) => val,
                Self::Goblin(val) => val,
                Self::HalfOrc(val) => val,
                Self::Human(val) => val,
                Self::Kobold(val) => val,
                Self::Halfling(val) => val,
            },
            self.feature()
        )
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Common => LANG_PACK.languages.common.clone(),
                Language::Dwarwish => LANG_PACK.languages.dwarwish.clone(),
                Language::Elvish => LANG_PACK.languages.elvish.clone(),
                Language::Sylvan => LANG_PACK.languages.sylvan.clone(),
                Language::Goblin => LANG_PACK.languages.goblin.clone(),
                Language::Orchish => LANG_PACK.languages.orchish.clone(),
                Language::Draconic => LANG_PACK.languages.draconic.clone(),
                Language::Giant => LANG_PACK.languages.giant.clone(),
                Language::Merran => LANG_PACK.languages.merran.clone(),
                Language::Reptillian => LANG_PACK.languages.reptillian.clone(),
                Language::Thanian => LANG_PACK.languages.thanian.clone(),
                Language::Celestial => LANG_PACK.languages.celestial.clone(),
                Language::Diabolic => LANG_PACK.languages.diabolic.clone(),
                Language::Primordial => LANG_PACK.languages.primordial.clone(),
                Language::AnyOf(list) => list
                    .iter()
                    .map(|l| format!("{l}"))
                    .collect::<Vec<String>>()
                    .join(&format!(" {} ", LANG_PACK.or)),
            }
        )
    }
}

#[derive(Debug, Deserialize, EnumIter, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Common,
    Dwarwish,
    Elvish,
    Sylvan,
    Goblin,
    Orchish,
    Draconic,
    Giant,
    Merran,
    Reptillian,
    Thanian,
    Celestial,
    Diabolic,
    Primordial,
    AnyOf(Vec<Self>),
}

impl Language {
    fn is_common(&self) -> bool {
        match self {
            Self::Common => true,
            Self::Dwarwish => true,
            Self::Elvish => true,
            Self::Sylvan => true,
            Self::Goblin => true,
            Self::Orchish => true,
            Self::Giant => true,
            Self::Merran => true,
            Self::Reptillian => true,
            Self::Thanian => true,
            Self::Draconic => false,
            Self::Celestial => false,
            Self::Diabolic => false,
            Self::Primordial => false,
            Self::AnyOf(list) => list.iter().any(|l| l.is_common()),
        }
    }

    fn roll_common() -> Self {
        let mut rng = rand::rng();
        let mut result = Language::iter().choose(&mut rng).unwrap();
        while !result.is_common() {
            result = Language::iter().choose(&mut rng).unwrap();
        }
        result
    }
}

enum AncestryFeature {
    Stout,
    Farsight,
    KeenSenses,
    Mighty,
    Stealthy,
    Ambitious,
    Knack,
}

impl Display for AncestryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let translation = match self {
            Self::Stout => LANG_PACK.ancestry_features.stout.clone(),
            Self::Farsight => LANG_PACK.ancestry_features.farsight.clone(),
            Self::KeenSenses => LANG_PACK.ancestry_features.keen_senses.clone(),
            Self::Mighty => LANG_PACK.ancestry_features.mighty.clone(),
            Self::Stealthy => LANG_PACK.ancestry_features.stealthy.clone(),
            Self::Ambitious => LANG_PACK.ancestry_features.ambitious.clone(),
            Self::Knack => LANG_PACK.ancestry_features.knack.clone(),
        };
        write!(f, "{}: {}", translation.name, translation.description)
    }
}
