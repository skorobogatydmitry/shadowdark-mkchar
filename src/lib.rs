use std::{collections::BTreeMap, fmt::Display};

use alignment::Alignment;
use background::Background;
use clap::Parser;

use ancestry::Ancestry;
use class::{Class, ClassAttributes};
use deities::Deity;
use inventory::Inventory;
use rand::{Rng, seq::IteratorRandom};
use strum::IntoEnumIterator;

mod alignment;
mod ancestry;
mod background;
mod class;
mod deities;
mod inventory;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum StatKind {
    Strength = 1,
    Dexterity = 2,
    Constitution = 3,
    Intellegence = 4,
    Wisdom = 5,
    Charisma = 6,
}

impl StatKind {
    fn roll(self) -> Stat {
        Stat {
            kind: self,
            val: Dice::D6.roll() + Dice::D6.roll() + Dice::D6.roll(),
        }
    }
}

#[derive(Debug)]
struct Stat {
    val: u8,
    kind: StatKind,
}

impl Stat {
    fn modifier(&self) -> i8 {
        (self.val as i8 - 10) as i8 / 2
    }

    fn name(&self) -> String {
        match self.kind {
            StatKind::Strength => langpack::PACK.strength.clone(),
            StatKind::Dexterity => langpack::PACK.dexterity.clone(),
            StatKind::Constitution => langpack::PACK.constitution.clone(),
            StatKind::Intellegence => langpack::PACK.intellegence.clone(),
            StatKind::Wisdom => langpack::PACK.wisdom.clone(),
            StatKind::Charisma => langpack::PACK.charisma.clone(),
        }
    }
}

impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} ({})", &self.name(), self.modifier(), self.val)
    }
}

#[derive(Debug)]
struct Stats {
    map: BTreeMap<StatKind, Stat>,
}

impl Stats {
    pub fn generate() -> Self {
        let mut attrs = vec![
            StatKind::Strength.roll(),
            StatKind::Dexterity.roll(),
            StatKind::Constitution.roll(),
            StatKind::Intellegence.roll(),
            StatKind::Wisdom.roll(),
            StatKind::Charisma.roll(),
        ];
        for _ in 0..10 {
            // let's say we roll 14+ in 10 attempts
            if attrs.iter().map(|a| a.val).max().unwrap() >= 14 {
                break;
            }
            attrs = vec![
                StatKind::Strength.roll(),
                StatKind::Dexterity.roll(),
                StatKind::Constitution.roll(),
                StatKind::Intellegence.roll(),
                StatKind::Wisdom.roll(),
                StatKind::Charisma.roll(),
            ];
        }

        if attrs.iter().map(|a| a.val).max().unwrap() < 14 {
            panic!("{}", langpack::PACK.error_messages.stats_out_of_attempts);
        }

        let map = attrs.into_iter().map(|a| (a.kind.clone(), a)).collect();
        Self { map }
    }

    pub fn modifier(&self, kind: StatKind) -> i8 {
        self.map.get(&kind).map(|s| s.modifier()).unwrap()
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.map
                .values()
                .map(|a| format!("{a}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

struct Character {
    max_hit_points: u32,
    background: Background,
    alignment: Alignment,
    deity: Deity,
    inventory: Inventory,
    name: String,
}

impl Character {
    fn new(stats: Stats, ancestry: Ancestry, class_attributes: ClassAttributes) -> Self {
        let alignment: Alignment = Alignment::iter().choose(&mut rand::rng()).unwrap();
        Self {
            max_hit_points: std::cmp::max(
                1,
                class_attributes.hit_points as i8 + stats.modifier(StatKind::Constitution) as i8,
            ) as u32,
            background: Background::iter().choose(&mut rand::rng()).unwrap(),
            deity: Deity::roll(&alignment),
            alignment,
            inventory: Inventory::new(class_attributes.level),
            name: langpack::PACK.names.roll(ancestry),
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}\n", langpack::PACK.name, self.name)?;
        write!(
            f,
            "{}: {}\n",
            langpack::PACK.hit_points,
            self.max_hit_points
        )?;
        write!(f, "{}\n", self.background)?;
        write!(f, "{}\n", self.alignment)?;
        write!(f, "{}\n", self.deity)?;
        write!(f, "{}", self.inventory)
    }
}

pub fn make_character(args: Args) {
    let stats = Stats::generate();
    println!("{} | {stats}", langpack::PACK.stats);
    let ancestry = Ancestry::roll();
    println!("{}", ancestry);
    let class = ClassArg::from(args.class).choose();
    println!(
        "{}: {}",
        langpack::PACK.class,
        class
            .as_ref()
            .map(|c| format!("{}", c))
            .unwrap_or(format!("{}", langpack::PACK.class_args.zero))
    );
    let class_attributes = class.map(|c| c.fill()).unwrap_or_default();
    println!("{}", class_attributes);

    let character = Character::new(stats, ancestry, class_attributes);
    println!("{}", character)
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
