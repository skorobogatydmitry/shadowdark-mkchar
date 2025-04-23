use std::{collections::BTreeMap, fmt::Display};

use alignment::Alignment;
use background::Background;
use clap::Parser;

use ancestry::Ancestry;
use class::{Class, ClassAttributes};
use deities::Deity;
use inventory::Inventory;
use rand::{Rng, seq::IteratorRandom};
use stats::{StatKind, Stats};
use strum::IntoEnumIterator;

mod alignment;
mod ancestry;
mod background;
mod class;
mod deities;
mod inventory;
mod stats;

mod langpack;

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
    max_hit_points: u32,
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
            max_hit_points: std::cmp::max(
                1,
                class_attributes.hit_points as i8 + stats.modifier(StatKind::Constitution) as i8,
            ) as u32,
            background: Background::iter().choose(&mut rand::rng()).unwrap(),
            deity: Deity::roll(&alignment),
            alignment,
            inventory: Inventory::new(class_attributes.level),
            name: langpack::PACK.names.roll(&ancestry),
            stats,
            ancestry,
            class_attributes,
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}\n", langpack::PACK.name, self.name)?;
        write!(f, "{}\n", self.ancestry)?;
        write!(
            f,
            "{}: {}\n",
            langpack::PACK.hit_points,
            self.max_hit_points
        )?;
        write!(f, "{}\n", self.background)?;
        write!(f, "{}\n", self.alignment)?;
        write!(f, "{}\n", self.deity)?;
        write!(f, "{}\n", self.stats)?;
        write!(f, "{}\n", self.class_attributes)?;
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
                Self::Zero => langpack::PACK.class_args.zero.clone(),
                Self::Any => langpack::PACK.class_args.any.clone(),
                Self::Fighter => langpack::PACK.class_args.fighter.clone(),
                Self::Thief => langpack::PACK.class_args.thief.clone(),
                Self::Wizard => langpack::PACK.class_args.wizard.clone(),
                Self::Priest => langpack::PACK.class_args.priest.clone(),
            }
        )
    }
}

impl From<String> for ClassArg {
    fn from(value: String) -> Self {
        if &value == langpack::PACK.class_args.zero.as_str() {
            Self::Zero
        } else if value == langpack::PACK.class_args.any.as_str() || value == String::default() {
            Self::Any
        } else if value == langpack::PACK.class_args.fighter.as_str() {
            Self::Fighter
        } else if value == langpack::PACK.class_args.thief.as_str() {
            Self::Thief
        } else if value == langpack::PACK.class_args.wizard.as_str() {
            Self::Wizard
        } else if value == langpack::PACK.class_args.priest.as_str() {
            Self::Priest
        } else {
            panic!(
                "{}: `{}'",
                langpack::PACK.error_messages.unknown_class_option,
                value
            );
        }
    }
}
