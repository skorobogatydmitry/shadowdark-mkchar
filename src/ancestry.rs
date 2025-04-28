use std::fmt::Display;

use clap::ValueEnum;
use rand::prelude::*;
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::translation::{Feature, LANG_PACK};

pub struct AncestryAttributes {
    pub ancestry: Ancestry,
    pub languages: Vec<Language>, // is only used for humans so far
}

impl AncestryAttributes {
    pub fn new(ancestry: Ancestry, chosen_language: Option<&Language>) -> Self {
        Self {
            languages: match ancestry {
                Ancestry::Dwarf => vec![Language::Common, Language::Dwarwish],
                Ancestry::Kobold => vec![Language::Common, Language::Draconic],
                Ancestry::Elf => vec![Language::Common, Language::Elvish, Language::Sylvan],
                Ancestry::Goblin => vec![Language::Common, Language::Goblin],
                Ancestry::Halfling => vec![Language::Common],
                Ancestry::HalfOrc => vec![Language::Common, Language::Orchish],
                Ancestry::Human => vec![
                    Language::Common,
                    chosen_language
                        .unwrap_or(
                            &ancestry
                                .allowed_extra_languages()
                                .choose(&mut rand::rng())
                                .unwrap(),
                        )
                        .clone(),
                ],
            },
            ancestry,
        }
    }
}

#[derive(Debug, EnumIter, Deserialize, Clone, ValueEnum, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Ancestry {
    Dwarf,
    Kobold,
    Elf,
    Goblin,
    Halfling,
    HalfOrc,
    Human,
}

impl Ancestry {
    pub fn roll() -> Self {
        Self::iter().choose(&mut rand::rng()).unwrap()
    }

    pub fn feature(&self) -> AncestryFeature {
        match self {
            Self::Dwarf => AncestryFeature::Stout,
            Self::Kobold => AncestryFeature::Knack,
            Self::Elf => AncestryFeature::Farsight,
            Self::Goblin => AncestryFeature::KeenSenses,
            Self::Halfling => AncestryFeature::Stealthy,
            Self::HalfOrc => AncestryFeature::Mighty,
            Self::Human => AncestryFeature::Ambitious,
        }
    }

    // for human's trait
    pub fn allowed_extra_languages(&self) -> Vec<Language> {
        if *self == Self::Human {
            Language::iter()
                .filter(|l| l.is_common() && *l != Language::Common)
                .collect()
        } else {
            vec![]
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Dwarf => LANG_PACK.ancestries.dwarf.clone(),
            Self::Elf => LANG_PACK.ancestries.elf.clone(),
            Self::Goblin => LANG_PACK.ancestries.goblin.clone(),
            Self::HalfOrc => LANG_PACK.ancestries.half_orc.clone(),
            Self::Human => LANG_PACK.ancestries.human.clone(),
            Self::Kobold => LANG_PACK.ancestries.kobold.clone(),
            Self::Halfling => LANG_PACK.ancestries.halfling.clone(),
        }
    }
}

impl Display for Ancestry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} | {}",
            LANG_PACK.ancestry,
            self.name(),
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
            }
        )
    }
}

#[derive(Debug, Deserialize, EnumIter, PartialEq, Clone, ValueEnum)]
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
}

impl Language {
    pub fn is_common(&self) -> bool {
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
        }
    }
}

pub enum AncestryFeature {
    Stout,
    Farsight,
    KeenSenses,
    Mighty,
    Stealthy,
    Ambitious,
    Knack,
}

impl AncestryFeature {
    pub fn to_feature(&self) -> Feature {
        match self {
            Self::Stout => LANG_PACK.ancestry_features.stout.clone(),
            Self::Farsight => LANG_PACK.ancestry_features.farsight.clone(),
            Self::KeenSenses => LANG_PACK.ancestry_features.keen_senses.clone(),
            Self::Mighty => LANG_PACK.ancestry_features.mighty.clone(),
            Self::Stealthy => LANG_PACK.ancestry_features.stealthy.clone(),
            Self::Ambitious => LANG_PACK.ancestry_features.ambitious.clone(),
            Self::Knack => LANG_PACK.ancestry_features.knack.clone(),
        }
    }
}

impl Display for AncestryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_feature())
    }
}
