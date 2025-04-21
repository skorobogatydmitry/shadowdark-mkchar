use std::{fmt::Display, vec};

use rand::seq::IndexedRandom;
use strum_macros::EnumIter;

use crate::{Dice, ancestry::LanguageKind, langpack};

#[derive(Debug, Clone, EnumIter)]
pub enum Class {
    Fighter,
    Thief,
    Wizard,
    Priest,
}

impl Class {
    pub fn fill(self) -> ClassAttributes {
        ClassAttributes {
            hit_points: match self {
                Self::Fighter => Dice::D8.roll(),
                Self::Thief => Dice::D4.roll(),
                Self::Priest => Dice::D6.roll(),
                Self::Wizard => Dice::D4.roll(),
            } as u32,
            weapon_masteries: match self {
                Self::Fighter => vec![WeaponMastery::All],
                Self::Thief => vec![
                    WeaponMastery::Club,
                    WeaponMastery::Crossbow,
                    WeaponMastery::Dagger,
                    WeaponMastery::Shortbow,
                    WeaponMastery::Shortsword,
                ],
                Self::Priest => vec![
                    WeaponMastery::Club,
                    WeaponMastery::Crossbow,
                    WeaponMastery::Mace,
                    WeaponMastery::Longsword,
                    WeaponMastery::Staff,
                    WeaponMastery::Warhammer,
                    WeaponMastery::Dagger,
                ],
                Self::Wizard => vec![WeaponMastery::Dagger, WeaponMastery::Staff],
            },
            armor_masteries: match self {
                Self::Fighter => vec![ArmorMastery::All, ArmorMastery::Shields],
                Self::Thief => vec![ArmorMastery::LeatherArmor, ArmorMastery::MithralChainmail],
                Self::Priest => vec![ArmorMastery::All, ArmorMastery::Shields],
                Self::Wizard => vec![],
            },
            class_features: match self {
                Self::Fighter => vec![
                    ClassFeature::Hauler,
                    ClassFeature::WeaponMastery,
                    ClassFeature::Grit,
                ],
                Self::Thief => vec![
                    ClassFeature::Theivery,
                    ClassFeature::JackOfAllTrades,
                    ClassFeature::Backstab,
                ],
                Self::Priest => vec![ClassFeature::SpellCasting, ClassFeature::TurnUndead],
                Self::Wizard => vec![ClassFeature::SpellCasting, ClassFeature::LearningSpells],
            },
            talents: vec![self.roll_talent()],
            languages: match self {
                Class::Priest => vec![
                    vec![
                        LanguageKind::Celestial,
                        LanguageKind::Diabolic,
                        LanguageKind::Primordial,
                    ]
                    .choose(&mut rand::rng())
                    .unwrap()
                    .clone(),
                ],
                _ => vec![],
            },
        }
    }

    fn roll_talent(&self) -> Talent {
        let talent_roll = Dice::D6.roll() + Dice::D6.roll();
        match self {
            Self::Fighter => match talent_roll {
                2 => Talent::WeaponMastery,
                3..=6 => Talent::PreciseStrike,
                7..=9 => Talent::Trained,
                10..=11 => Talent::ArmorTraining,
                12 => Talent::Gifted,
                _ => panic!("2d6 == {} !!!", talent_roll),
            },
            Self::Thief => match talent_roll {
                2 => Talent::Vigilant, // TODO: re-roll dups
                3..=5 => Talent::DeadlyStab,
                6..=9 => Talent::Versatile,
                10..=11 => Talent::PreciseStrike,
                12 => Talent::Gifted,
                _ => panic!("2d6 == {} !!!", talent_roll),
            },
            Self::Priest => match talent_roll {
                2 => Talent::SpellExpert,
                3..=6 => Talent::PreciseStrike,
                7..=9 => Talent::GodBlessed,
                10..=11 => Talent::Devoted,
                12 => Talent::Gifted,
                _ => panic!("2d6 == {} !!!", talent_roll),
            },
            Self::Wizard => match talent_roll {
                2 => Talent::ThinAirCraft,
                3..=7 => Talent::SkilledCaster,
                8..=9 => Talent::SpellExpert,
                10..=11 => Talent::Bookworm,
                12 => Talent::Gifted,
                _ => panic!("2d6 == {} !!!", talent_roll),
            },
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fighter => langpack::PACK.class_args.fighter.clone(),
                Self::Thief => langpack::PACK.class_args.thief.clone(),
                Self::Wizard => langpack::PACK.class_args.wizard.clone(),
                Self::Priest => langpack::PACK.class_args.priest.clone(),
            }
        )
    }
}

pub struct ClassAttributes {
    pub hit_points: u32,
    pub weapon_masteries: Vec<WeaponMastery>,
    pub armor_masteries: Vec<ArmorMastery>,
    pub class_features: Vec<ClassFeature>,
    pub talents: Vec<Talent>,
    pub languages: Vec<LanguageKind>,
}

impl Display for ClassAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}\n{}: {}\n{}: {}\n{}:\n  {}\n{}:\n  {}",
            langpack::PACK.hit_points,
            self.hit_points,
            langpack::PACK.weapon,
            self.weapon_masteries
                .iter()
                .map(|m| format!("{}", m))
                .collect::<Vec<String>>()
                .join(", "),
            langpack::PACK.armor,
            self.armor_masteries
                .iter()
                .map(|m| format!("{}", m))
                .collect::<Vec<String>>()
                .join(", "),
            langpack::PACK.features,
            self.class_features
                .iter()
                .map(|cf| format!("{}", cf))
                .collect::<Vec<String>>()
                .join("\n  "),
            langpack::PACK.talent,
            self.talents
                .iter()
                .map(|cf| format!("{}", cf))
                .collect::<Vec<String>>()
                .join("\n  "),
        )
    }
}

impl Default for ClassAttributes {
    // 0-level
    fn default() -> Self {
        Self {
            hit_points: 0,
            weapon_masteries: vec![WeaponMastery::All],
            armor_masteries: vec![ArmorMastery::All, ArmorMastery::Shields],
            class_features: vec![ClassFeature::BeginnersLuck],
            talents: vec![],
            languages: vec![],
        }
    }
}

pub enum WeaponMastery {
    All,
    Club,
    Crossbow,
    Mace,
    Longsword,
    Staff,
    Warhammer,
    Dagger,
    Shortbow,
    Shortsword,
}

impl Display for WeaponMastery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::All => langpack::PACK.weapon_masteries.all.clone(),
                Self::Club => langpack::PACK.weapon_masteries.club.clone(),
                Self::Crossbow => langpack::PACK.weapon_masteries.crossbow.clone(),
                Self::Mace => langpack::PACK.weapon_masteries.mace.clone(),
                Self::Longsword => langpack::PACK.weapon_masteries.longsword.clone(),
                Self::Staff => langpack::PACK.weapon_masteries.staff.clone(),
                Self::Warhammer => langpack::PACK.weapon_masteries.warhammer.clone(),
                Self::Dagger => langpack::PACK.weapon_masteries.dagger.clone(),
                Self::Shortbow => langpack::PACK.weapon_masteries.shortbow.clone(),
                Self::Shortsword => langpack::PACK.weapon_masteries.shortsword.clone(),
            }
        )
    }
}

pub enum ArmorMastery {
    All,
    Shields,
    LeatherArmor,
    MithralChainmail,
}

impl Display for ArmorMastery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::All => langpack::PACK.armor_masteries.all.clone(),
                Self::Shields => langpack::PACK.armor_masteries.shields.clone(),
                Self::LeatherArmor => langpack::PACK.armor_masteries.leather_armor.clone(),
                Self::MithralChainmail => langpack::PACK.armor_masteries.mithral_chainmail.clone(),
            }
        )
    }
}

pub enum ClassFeature {
    Hauler,
    WeaponMastery,
    Grit,
    TurnUndead,
    SpellCasting,
    LearningSpells,
    Backstab,
    Theivery,
    JackOfAllTrades,
    BeginnersLuck,
}

impl Display for ClassFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let feature = match self {
            Self::Hauler => langpack::PACK.class_features.hauler.clone(),
            Self::WeaponMastery => langpack::PACK.class_features.weapon_mastery.clone(),
            Self::Grit => langpack::PACK.class_features.grit.clone(),
            Self::TurnUndead => langpack::PACK.class_features.turn_undead.clone(),
            Self::SpellCasting => langpack::PACK.class_features.spellcasting.clone(),
            Self::LearningSpells => langpack::PACK.class_features.learning_spells.clone(),
            Self::Backstab => langpack::PACK.class_features.backstab.clone(),
            Self::Theivery => langpack::PACK.class_features.theivery.clone(),
            Self::JackOfAllTrades => langpack::PACK.class_features.jack_of_all_trades.clone(),
            Self::BeginnersLuck => langpack::PACK.class_features.beginners_luck.clone(),
        };
        write!(f, "{}: {}", feature.name, feature.description)
    }
}

// all names are made-up by me as convenient lables
pub enum Talent {
    WeaponMastery,
    PreciseStrike,
    Trained,
    ArmorTraining,
    Gifted,
    Versatile, // Thief stats
    Vigilant,
    DeadlyStab,
    GodBlessed,
    SpellExpert,
    SkilledCaster,
    Devoted,
    ThinAirCraft,
    Bookworm,
}

impl Display for Talent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::WeaponMastery => langpack::PACK.talents.weapon_mastery.clone(),
                Self::PreciseStrike => langpack::PACK.talents.precise_strike.clone(),
                Self::Trained => langpack::PACK.talents.trained.clone(),
                Self::ArmorTraining => langpack::PACK.talents.armor_training.clone(),
                Self::Gifted => langpack::PACK.talents.gifted.clone(),
                Self::Versatile => langpack::PACK.talents.versatile.clone(),
                Self::Vigilant => langpack::PACK.talents.vigilant.clone(),
                Self::DeadlyStab => langpack::PACK.talents.deadly_stab.clone(),
                Self::GodBlessed => langpack::PACK.talents.god_blessed.clone(),
                Self::SpellExpert => langpack::PACK.talents.spell_expert.clone(),
                Self::SkilledCaster => langpack::PACK.talents.skilled_caster.clone(),
                Self::Devoted => langpack::PACK.talents.devoted.clone(),
                Self::ThinAirCraft => langpack::PACK.talents.thin_air_craft.clone(),
                Self::Bookworm => langpack::PACK.talents.bookworm.clone(),
            }
        )
    }
}
