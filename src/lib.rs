use std::fmt::Display;

use alignment::Alignment;
use background::Background;
use clap::Parser;

use ancestry::{Ancestry, Language};
use class::{Class, ClassAttributes};
use deities::Deity;
use inventory::Inventory;
use rand::{Rng, seq::IteratorRandom};
use stats::{StatKind, Stats};
use strum::IntoEnumIterator;
use translation::LANG_PACK;

mod alignment;
mod ancestry;
mod background;
mod class;
mod deities;
mod inventory;
mod stats;

mod translation;

enum Dice {
    D6,
    D8,
    D4,
}

impl Dice {
    fn roll(&self) -> u8 {
        let max = match self {
            Self::D4 => 4,
            Self::D6 => 6,
            Self::D8 => 8,
        };
        rand::rng().random_range(1..=max)
    }
}

pub struct Character {
    background: Background,
    alignment: Alignment,
    deity: Deity,
    inventory: Inventory,
    name: String,
    stats: Stats,
    ancestry: Ancestry,
    class_attributes: ClassAttributes,
}

impl Character {
    pub fn new(args: Args) -> Self {
        let class = ClassArg::from(args.class).choose();
        let class_attributes = class.map(|c| c.fill()).unwrap_or_default();
        let alignment: Alignment = Alignment::iter().choose(&mut rand::rng()).unwrap();
        let ancestry = Ancestry::roll();
        let stats = Stats::generate();
        Self {
            background: Background::iter().choose(&mut rand::rng()).unwrap(),
            deity: Deity::roll(&alignment),
            alignment,
            inventory: Inventory::new(class_attributes.level),
            name: LANG_PACK.names.roll(&ancestry),
            stats,
            ancestry,
            class_attributes,
        }
    }

    pub fn max_hit_points(&self) -> u32 {
        std::cmp::max(
            1,
            self.class_attributes.hit_points as i8
                + self.stats.modifier(StatKind::Constitution) as i8,
        ) as u32
    }

    pub fn languages(&self) -> Vec<Language> {
        let mut all_languages = vec![];
        all_languages.extend(self.ancestry.languages());
        all_languages.extend(self.class_attributes.languages.clone());
        all_languages
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", LANG_PACK.name, self.name)?;
        writeln!(f, "{}", self.ancestry)?;
        writeln!(f, "{}: {}", LANG_PACK.hit_points, self.max_hit_points())?;
        writeln!(f, "{}", self.background)?;
        writeln!(f, "{}", self.alignment)?;
        writeln!(f, "{}", self.deity)?;
        writeln!(
            f,
            "{}: {}",
            LANG_PACK.language,
            self.languages()
                .into_iter()
                .map(|l| format!("{l}"))
                .collect::<Vec<String>>()
                .join(", "),
        )?;
        writeln!(f, "{}", self.stats)?;
        writeln!(f, "{}", self.class_attributes)?;
        write!(f, "{}", self.inventory)
    }
}

/// Shadowdark quick characters generator
/// supports custom translations
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value_t = String::default())]
    class: String,
}

#[derive(Debug)]
pub enum ClassArg {
    Zero,
    Any,
    Fighter,
    Thief,
    Wizard,
    Priest,
}

impl ClassArg {
    fn choose(self) -> Option<Class> {
        match self {
            Self::Zero => None,
            Self::Fighter => Some(Class::Fighter),
            Self::Thief => Some(Class::Thief),
            Self::Wizard => Some(Class::Wizard),
            Self::Priest => Some(Class::Priest),
            Self::Any => Some(Class::iter().choose(&mut rand::rng()).unwrap()),
        }
    }
}

impl Display for ClassArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Zero => LANG_PACK.class_args.zero.clone(),
                Self::Any => LANG_PACK.class_args.any.clone(),
                Self::Fighter => LANG_PACK.class_args.fighter.clone(),
                Self::Thief => LANG_PACK.class_args.thief.clone(),
                Self::Wizard => LANG_PACK.class_args.wizard.clone(),
                Self::Priest => LANG_PACK.class_args.priest.clone(),
            }
        )
    }
}

impl From<String> for ClassArg {
    fn from(value: String) -> Self {
        if &value == LANG_PACK.class_args.zero.as_str() {
            Self::Zero
        } else if value == LANG_PACK.class_args.any.as_str() || value == String::default() {
            Self::Any
        } else if value == LANG_PACK.class_args.fighter.as_str() {
            Self::Fighter
        } else if value == LANG_PACK.class_args.thief.as_str() {
            Self::Thief
        } else if value == LANG_PACK.class_args.wizard.as_str() {
            Self::Wizard
        } else if value == LANG_PACK.class_args.priest.as_str() {
            Self::Priest
        } else {
            panic!(
                "{}: `{}'",
                LANG_PACK.error_messages.unknown_class_option, value
            );
        }
    }
}
