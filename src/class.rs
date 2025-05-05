use std::{fmt::Display, vec};

use rand::seq::IndexedRandom;
use strum_macros::EnumIter;

use crate::{
    Dice,
    ancestry::Language,
    args::ARGS,
    spell::Spell,
    translation::{Feature, LANG_PACK},
};

#[derive(Debug, Clone, EnumIter, PartialEq, Eq)]
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
            level: 1,
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
                Self::Priest => vec![ClassFeature::choose_spells(&self), ClassFeature::TurnUndead],
                Self::Wizard => vec![
                    ClassFeature::choose_spells(&self),
                    ClassFeature::LearningSpells,
                ],
            },
            talents: vec![self.roll_talent()],
            languages: match self {
                Class::Priest => vec![
                    vec![
                        Language::Celestial,
                        Language::Diabolic,
                        Language::Primordial,
                    ]
                    .choose(&mut rand::rng())
                    .unwrap()
                    .clone(),
                ],
                _ => vec![],
            },
            class: Some(self),
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
                Self::Fighter => LANG_PACK.class_args.fighter.clone(),
                Self::Thief => LANG_PACK.class_args.thief.clone(),
                Self::Wizard => LANG_PACK.class_args.wizard.clone(),
                Self::Priest => LANG_PACK.class_args.priest.clone(),
            }
        )
    }
}

pub struct ClassAttributes {
    pub hit_points: u32,
    pub level: u8,
    pub weapon_masteries: Vec<WeaponMastery>,
    pub armor_masteries: Vec<ArmorMastery>,
    pub class_features: Vec<ClassFeature>,
    pub talents: Vec<Talent>,
    pub languages: Vec<Language>,
    pub class: Option<Class>,
}

impl ClassAttributes {
    pub fn class_name(&self) -> String {
        self.class
            .as_ref()
            .map(|c| format!("{}", c))
            .unwrap_or(LANG_PACK.class_args.zero.clone())
    }
}

impl Display for ClassAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}\n{}: {}\n{}: {}\n{}:\n  {}\n{}:\n  {}",
            LANG_PACK.class,
            self.class_name(),
            LANG_PACK.weapon,
            self.weapon_masteries
                .iter()
                .map(|m| format!("{}", m))
                .collect::<Vec<String>>()
                .join(", "),
            LANG_PACK.armor,
            self.armor_masteries
                .iter()
                .map(|m| format!("{}", m))
                .collect::<Vec<String>>()
                .join(", "),
            LANG_PACK.features,
            self.class_features
                .iter()
                .map(|cf| format!("{}", cf))
                .collect::<Vec<String>>()
                .join("\n  "),
            LANG_PACK.talent,
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
            level: 0,
            class: None,
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

impl WeaponMastery {
    pub fn name(&self) -> String {
        match self {
            Self::All => LANG_PACK.weapon_masteries.all.clone(),
            Self::Club => LANG_PACK.weapon_masteries.club.clone(),
            Self::Crossbow => LANG_PACK.weapon_masteries.crossbow.clone(),
            Self::Mace => LANG_PACK.weapon_masteries.mace.clone(),
            Self::Longsword => LANG_PACK.weapon_masteries.longsword.clone(),
            Self::Staff => LANG_PACK.weapon_masteries.staff.clone(),
            Self::Warhammer => LANG_PACK.weapon_masteries.warhammer.clone(),
            Self::Dagger => LANG_PACK.weapon_masteries.dagger.clone(),
            Self::Shortbow => LANG_PACK.weapon_masteries.shortbow.clone(),
            Self::Shortsword => LANG_PACK.weapon_masteries.shortsword.clone(),
        }
    }
}

impl Display for WeaponMastery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub enum ArmorMastery {
    All,
    Shields,
    LeatherArmor,
    MithralChainmail,
}

impl ArmorMastery {
    pub fn name(&self) -> String {
        match self {
            Self::All => LANG_PACK.armor_masteries.all.clone(),
            Self::Shields => LANG_PACK.armor_masteries.shields.clone(),
            Self::LeatherArmor => LANG_PACK.armor_masteries.leather_armor.clone(),
            Self::MithralChainmail => LANG_PACK.armor_masteries.mithral_chainmail.clone(),
        }
    }
}

impl Display for ArmorMastery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub enum ClassFeature {
    Hauler,
    WeaponMastery,
    Grit,
    TurnUndead,
    SpellCasting(Vec<Spell>),
    LearningSpells,
    Backstab,
    Theivery,
    JackOfAllTrades,
    BeginnersLuck,
}

impl ClassFeature {
    pub fn choose_spells(c: &Class) -> Self {
        let spells_count = match c {
            Class::Priest => 2,
            Class::Wizard => 3,
            _ => 0,
        };
        ClassFeature::SpellCasting(
            if let Some(spell_list) = ARGS.spell.as_ref().map(|s| s.to_vec()) {
                for s in &spell_list {
                    if !s.of_class(c) {
                        panic!(
                            "{}: {} {}",
                            LANG_PACK.error_messages.incorrect_spell_class, c, s
                        )
                    }
                }
                if spell_list.len() != spells_count {
                    panic!(
                        "{}: {} / {}",
                        LANG_PACK.error_messages.incorrect_spells_count,
                        spells_count,
                        spell_list.len()
                    )
                }
                spell_list
            } else {
                Spell::roll(c, spells_count)
            },
        )
    }

    pub fn to_feature(&self) -> Feature {
        match self {
            Self::Hauler => LANG_PACK.class_features.hauler.clone(),
            Self::WeaponMastery => LANG_PACK.class_features.weapon_mastery.clone(),
            Self::Grit => LANG_PACK.class_features.grit.clone(),
            Self::TurnUndead => LANG_PACK.class_features.turn_undead.clone(),
            Self::SpellCasting(spells) => {
                let mut f = LANG_PACK.class_features.spellcasting.clone();
                f.description.push_str(&format!(
                    ": {}",
                    spells
                        .iter()
                        .map(|s| s.to_feature().name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
                f
            }
            Self::LearningSpells => LANG_PACK.class_features.learning_spells.clone(),
            Self::Backstab => LANG_PACK.class_features.backstab.clone(),
            Self::Theivery => LANG_PACK.class_features.theivery.clone(),
            Self::JackOfAllTrades => LANG_PACK.class_features.jack_of_all_trades.clone(),
            Self::BeginnersLuck => LANG_PACK.class_features.beginners_luck.clone(),
        }
    }
}

impl Display for ClassFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_feature())?;
        match self {
            ClassFeature::SpellCasting(spells) => {
                for s in spells {
                    write!(f, "\n\t{}", s)?;
                }
            }
            _ => (),
        }
        Ok(())
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
                Self::WeaponMastery => LANG_PACK.talents.weapon_mastery.clone(),
                Self::PreciseStrike => LANG_PACK.talents.precise_strike.clone(),
                Self::Trained => LANG_PACK.talents.trained.clone(),
                Self::ArmorTraining => LANG_PACK.talents.armor_training.clone(),
                Self::Gifted => LANG_PACK.talents.gifted.clone(),
                Self::Versatile => LANG_PACK.talents.versatile.clone(),
                Self::Vigilant => LANG_PACK.talents.vigilant.clone(),
                Self::DeadlyStab => LANG_PACK.talents.deadly_stab.clone(),
                Self::GodBlessed => LANG_PACK.talents.god_blessed.clone(),
                Self::SpellExpert => LANG_PACK.talents.spell_expert.clone(),
                Self::SkilledCaster => LANG_PACK.talents.skilled_caster.clone(),
                Self::Devoted => LANG_PACK.talents.devoted.clone(),
                Self::ThinAirCraft => LANG_PACK.talents.thin_air_craft.clone(),
                Self::Bookworm => LANG_PACK.talents.bookworm.clone(),
            }
        )
    }
}
