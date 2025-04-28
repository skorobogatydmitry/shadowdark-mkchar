use std::fmt::Display;

use alignment::Alignment;
use args::{ARGS, Args};
use background::Background;

use ancestry::{Ancestry, AncestryAttributes, Language};
use clap::CommandFactory;
use class::ClassAttributes;
use deities::Deity;
use inventory::Inventory;
use rand::Rng;
use stats::{StatKind, Stats};
use translation::LANG_PACK;

pub mod args;
pub mod template;

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
    ancestry_attributes: AncestryAttributes,
    class_attributes: ClassAttributes,
}

impl Character {
    pub fn new() -> Self {
        let stats = Stats::generate();
        let class = ARGS.class.choose(&stats);
        let class_attributes = class.map(|c| c.fill()).unwrap_or_default();
        let alignment: Alignment = ARGS
            .alignment
            .as_ref()
            .cloned()
            .unwrap_or(Alignment::roll());
        let ancestry = ARGS.ancestry.as_ref().cloned().unwrap_or(Ancestry::roll());
        // check the extra language supplied
        // TODO: find a better place
        if ARGS
            .language
            .as_ref()
            .map(|l| !ancestry.allowed_extra_languages().contains(l))
            .unwrap_or(false)
        {
            Args::command()
                .error(
                    clap::error::ErrorKind::InvalidValue,
                    format!(
                        "{} {}. {}: [{}]",
                        LANG_PACK.error_messages.non_common_language,
                        ancestry.name(),
                        LANG_PACK.available,
                        ancestry
                            .allowed_extra_languages()
                            .iter()
                            .map(|l| format!("{l}"))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ),
                )
                .exit();
        }
        let ancestry_attributes = AncestryAttributes::new(ancestry, ARGS.language.as_ref());
        Self {
            background: Background::roll(),
            deity: Deity::roll(&alignment),
            alignment,
            inventory: Inventory::new(class_attributes.level),
            name: ARGS
                .name
                .as_ref()
                .unwrap_or(LANG_PACK.names.roll(&ancestry_attributes.ancestry))
                .clone(),
            stats,
            ancestry_attributes,
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
        // TODO: avoid clonning, just return joined references
        all_languages.extend(self.ancestry_attributes.languages.clone());
        all_languages.extend(self.class_attributes.languages.clone());
        all_languages
    }

    pub fn inventory_slots_count(&self) -> u8 {
        std::cmp::max(10, self.stats.value(StatKind::Strength))
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", LANG_PACK.name, self.name)?;
        writeln!(f, "{}", self.ancestry_attributes.ancestry)?;
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
