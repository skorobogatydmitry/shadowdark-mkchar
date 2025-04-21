use std::fmt::Display;

use rand::prelude::*;
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::langpack;

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
        if Self::iter().len() != langpack::PACK.ancestries.len() {
            panic!("{}", langpack::PACK.error_messages.not_all_ancestries);
        }
        langpack::PACK
            .ancestries
            .choose(&mut rand::rng())
            .unwrap()
            .clone()
    }
    pub fn languages(&self) -> Vec<Language> {
        match self {
            Self::Dwarf(_) => vec![
                Language::get(LanguageKind::Common),
                Language::get(LanguageKind::Dwarwish),
            ],
            Self::Kobold(_) => vec![
                Language::get(LanguageKind::Common),
                Language::get(LanguageKind::Draconic),
            ],
            Self::Elf(_) => vec![
                Language::get(LanguageKind::Common),
                Language::get(LanguageKind::Elvish),
                Language::get(LanguageKind::Sylvan),
            ],
            Self::Goblin(_) => vec![
                Language::get(LanguageKind::Common),
                Language::get(LanguageKind::Goblin),
            ],
            Self::Halfling(_) => vec![Language::get(LanguageKind::Common)],
            Self::HalfOrc(_) => vec![
                Language::get(LanguageKind::Common),
                Language::get(LanguageKind::Orchish),
            ],
            Self::Human(_) => vec![
                Language::get(LanguageKind::Common),
                loop {
                    let lang = Language::roll_common();
                    if lang.kind != LanguageKind::Common {
                        break lang;
                    }
                },
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
            "{}: {} | {}: {} | {}",
            langpack::PACK.ancestry,
            match self {
                Self::Dwarf(val) => val,
                Self::Elf(val) => val,
                Self::Goblin(val) => val,
                Self::HalfOrc(val) => val,
                Self::Human(val) => val,
                Self::Kobold(val) => val,
                Self::Halfling(val) => val,
            },
            langpack::PACK.language,
            self.languages()
                .into_iter()
                .map(|l| format!("{l}"))
                .collect::<Vec<String>>()
                .join(", "),
            self.feature()
        )
    }
}

pub struct Language {
    kind: LanguageKind,
    name: String,
}

impl Language {
    fn get(kind: LanguageKind) -> Self {
        Self {
            name: match &kind {
                LanguageKind::Common => langpack::PACK.languages.common.clone(),
                LanguageKind::Dwarwish => langpack::PACK.languages.dwarwish.clone(),
                LanguageKind::Elvish => langpack::PACK.languages.elvish.clone(),
                LanguageKind::Sylvan => langpack::PACK.languages.sylvan.clone(),
                LanguageKind::Goblin => langpack::PACK.languages.goblin.clone(),
                LanguageKind::Orchish => langpack::PACK.languages.orchish.clone(),
                LanguageKind::Draconic => langpack::PACK.languages.draconic.clone(),
                LanguageKind::Giant => langpack::PACK.languages.giant.clone(),
                LanguageKind::Merran => langpack::PACK.languages.merran.clone(),
                LanguageKind::Reptillian => langpack::PACK.languages.reptillian.clone(),
                LanguageKind::Thanian => langpack::PACK.languages.thanian.clone(),
                LanguageKind::Celestial => langpack::PACK.languages.celestial.clone(),
                LanguageKind::Diabolic => langpack::PACK.languages.diabolic.clone(),
                LanguageKind::Primordial => langpack::PACK.languages.primordial.clone(),
            },
            kind,
        }
    }

    fn roll_common() -> Self {
        let mut rng = rand::rng();
        let mut kind = LanguageKind::iter().choose(&mut rng).unwrap();
        while !kind.is_common() {
            kind = LanguageKind::iter().choose(&mut rng).unwrap();
        }
        Self::get(kind)
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Deserialize, EnumIter, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LanguageKind {
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

impl LanguageKind {
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
        }
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
            Self::Stout => langpack::PACK.ancestry_features.stout.clone(),
            Self::Farsight => langpack::PACK.ancestry_features.farsight.clone(),
            Self::KeenSenses => langpack::PACK.ancestry_features.keen_senses.clone(),
            Self::Mighty => langpack::PACK.ancestry_features.mighty.clone(),
            Self::Stealthy => langpack::PACK.ancestry_features.stealthy.clone(),
            Self::Ambitious => langpack::PACK.ancestry_features.ambitious.clone(),
            Self::Knack => langpack::PACK.ancestry_features.knack.clone(),
        };
        write!(f, "{}: {}", translation.name, translation.description)
    }
}
