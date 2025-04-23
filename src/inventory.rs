use std::fmt::Display;

use rand::seq::IndexedRandom;

use crate::{Dice, langpack};

pub struct Inventory {
    purse: Purse,
    equipment: Vec<Gear>,
}

impl Inventory {
    pub fn new(level: u8) -> Self {
        if level == 0 {
            Self {
                purse: Purse::empty(),
                equipment: Gear::roll_zero_level_gear(),
            }
        } else {
            Self {
                purse: Purse::roll(),
                equipment: vec![],
            }
        }
    }
}

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n", langpack::PACK.inventory)?;
        for e in &self.equipment {
            write!(f, "  {}\n", e)?;
        }
        write!(f, "  {}", self.purse)?;
        Ok(())
    }
}

pub struct Purse {
    gold: u32,
    silver: u32,
}

impl Purse {
    fn roll() -> Self {
        Purse {
            gold: (Dice::D6.roll() as u32 + Dice::D6.roll() as u32) * 5,
            silver: 0,
        }
    }

    fn empty() -> Self {
        Self { gold: 0, silver: 0 }
    }
}

impl Display for Purse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} - {}, {} - {}",
            langpack::PACK.purse,
            langpack::PACK.gold_pieces,
            self.gold,
            langpack::PACK.silver_pieces,
            self.silver
        )
    }
}

#[derive(Debug, Clone)]
pub enum Gear {
    Torch,
    Dagger,
    Pole,
    Shortbow,
    Arrows(u8),
    Rope(u8),
    FlaskOfOil,
    Crowbar,
    IronSpikes(u8),
    FlintAndSteel,
    GrapplingHook,
    Club,
    BagOfCaltrops,
}

const ZERO_LEVEL_OPTIONS: [&[Gear]; 12] = [
    &[Gear::Torch],
    &[Gear::Dagger],
    &[Gear::Pole],
    &[Gear::Shortbow, Gear::Arrows(5)],
    &[Gear::Rope(60)],
    &[Gear::FlaskOfOil],
    &[Gear::Crowbar],
    &[Gear::IronSpikes(10)],
    &[Gear::FlintAndSteel],
    &[Gear::GrapplingHook],
    &[Gear::Club],
    &[Gear::BagOfCaltrops],
];

impl Gear {
    fn roll_zero_level_gear() -> Vec<Self> {
        let mut result = vec![];
        for _ in 0..Dice::D4.roll() {
            result.extend(
                ZERO_LEVEL_OPTIONS
                    .choose(&mut rand::rng())
                    .unwrap()
                    .iter()
                    .map(|g| (*g).clone()),
            );
        }
        result
    }
}

impl Display for Gear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Torch => write!(f, "{}", langpack::PACK.gear.torch),
            Self::Dagger => write!(f, "{}", langpack::PACK.gear.dagger),
            Self::Pole => write!(f, "{}", langpack::PACK.gear.pole),
            Self::Shortbow => write!(f, "{}", langpack::PACK.gear.shortbow),
            Self::Arrows(count) => write!(f, "{} ({})", langpack::PACK.gear.arrows, count),
            Self::Rope(len) => write!(f, "{} ({}′)", langpack::PACK.gear.rope, len),
            Self::FlaskOfOil => write!(f, "{}", langpack::PACK.gear.flask_of_oil),
            Self::Crowbar => write!(f, "{}", langpack::PACK.gear.crowbar),
            Self::IronSpikes(count) => write!(f, "{} ({})", langpack::PACK.gear.iron_spikes, count),
            Self::FlintAndSteel => write!(f, "{}", langpack::PACK.gear.flint_and_steel),
            Self::GrapplingHook => write!(f, "{}", langpack::PACK.gear.grappling_hook),
            Self::Club => write!(f, "{}", langpack::PACK.gear.club),
            Self::BagOfCaltrops => write!(f, "{}", langpack::PACK.gear.bag_of_caltrops),
        }
    }
}
