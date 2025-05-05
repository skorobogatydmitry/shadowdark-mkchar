use clap::ValueEnum;
use rand::seq::IndexedRandom;
use std::{fmt::Display, vec};
use strum_macros::EnumIter;

use crate::{
    Dice,
    class::Class,
    translation::{Feature, LANG_PACK},
};

static PRIEST_SPELLS: [Spell; 6] = [
    Spell::CureWounds,
    Spell::HolyWeapon,
    Spell::Light,
    Spell::ProtectionFromEvil,
    Spell::ShieldOfFaith,
    Spell::TurnUndead,
];

static WIZARD_SPELLS: [Spell; 12] = [
    Spell::Alarm,
    Spell::BurningHands,
    Spell::CharmPerson,
    Spell::DetectMagic,
    Spell::FeatherFall,
    Spell::FloatingDisk,
    Spell::HoldPortal,
    Spell::Light,
    Spell::MageArmor,
    Spell::MagicMissile,
    Spell::ProtectionFromEvil,
    Spell::Sleep,
];

#[derive(Debug, Clone, ValueEnum, EnumIter, PartialEq, Eq)]
pub enum Spell {
    CureWounds,
    HolyWeapon,
    Light,
    ProtectionFromEvil,
    ShieldOfFaith,
    TurnUndead,
    Alarm,
    BurningHands,
    CharmPerson,
    DetectMagic,
    FeatherFall,
    FloatingDisk,
    HoldPortal,
    MageArmor,
    MagicMissile,
    Sleep,
}

impl Spell {
    pub fn roll(class: &Class, count: usize) -> Vec<Spell> {
        let mut rng = rand::rng();
        let list = match class {
            Class::Priest => PRIEST_SPELLS.to_vec(),
            Class::Wizard => WIZARD_SPELLS.to_vec(),
            _ => vec![],
        };

        if list.len() < count {
            panic!(
                "{}: {} / {}",
                LANG_PACK.error_messages.not_enough_spells,
                list.len(),
                count
            )
        }

        list.choose_multiple(&mut rng, count)
            .map(|s| s.clone())
            .collect()
    }

    pub fn of_class(&self, class: &Class) -> bool {
        match class {
            Class::Priest => PRIEST_SPELLS.contains(self),
            Class::Wizard => WIZARD_SPELLS.contains(self),
            _ => false,
        }
    }

    pub fn level(&self) -> usize {
        match self {
            Self::CureWounds => 1,
            Self::HolyWeapon => 1,
            Self::Light => 1,
            Self::ProtectionFromEvil => 1,
            Self::ShieldOfFaith => 1,
            Self::TurnUndead => 1,
            Self::Alarm => 1,
            Self::BurningHands => 1,
            Self::CharmPerson => 1,
            Self::DetectMagic => 1,
            Self::FeatherFall => 1,
            Self::FloatingDisk => 1,
            Self::HoldPortal => 1,
            Self::MageArmor => 1,
            Self::MagicMissile => 1,
            Self::Sleep => 1,
        }
    }

    pub fn duration(&self) -> Duration {
        match self {
            Self::CureWounds => Duration::Instant,
            Self::HolyWeapon => Duration::Rounds(5),
            Self::Light => Duration::Hours(1),
            Self::ProtectionFromEvil => Duration::Focus,
            Self::ShieldOfFaith => Duration::Rounds(5),
            Self::TurnUndead => Duration::Instant,
            Self::Alarm => Duration::Days(1),
            Self::BurningHands => Duration::Instant,
            Self::CharmPerson => Duration::DaysDice(Dice::D8),
            Self::DetectMagic => Duration::Focus,
            Self::FeatherFall => Duration::Instant,
            Self::FloatingDisk => Duration::Rounds(10),
            Self::HoldPortal => Duration::Rounds(10),
            Self::MageArmor => Duration::Rounds(10),
            Self::MagicMissile => Duration::Instant,
            Self::Sleep => Duration::Instant,
        }
    }

    pub fn distance(&self) -> Distance {
        match self {
            Self::CureWounds => Distance::Close,
            Self::HolyWeapon => Distance::Close,
            Self::Light => Distance::Close,
            Self::ProtectionFromEvil => Distance::Close,
            Self::ShieldOfFaith => Distance::OnSelf,
            Self::TurnUndead => Distance::Near,
            Self::Alarm => Distance::Close,
            Self::BurningHands => Distance::Close,
            Self::CharmPerson => Distance::Near,
            Self::DetectMagic => Distance::Near,
            Self::FeatherFall => Distance::OnSelf,
            Self::FloatingDisk => Distance::Near,
            Self::HoldPortal => Distance::Near,
            Self::MageArmor => Distance::OnSelf,
            Self::MagicMissile => Distance::Far,
            Self::Sleep => Distance::Near,
        }
    }

    pub fn to_feature(&self) -> Feature {
        match self {
            Self::CureWounds => LANG_PACK.spells.cure_wounds.clone(),
            Self::HolyWeapon => LANG_PACK.spells.holy_weapon.clone(),
            Self::Light => LANG_PACK.spells.light.clone(),
            Self::ProtectionFromEvil => LANG_PACK.spells.protection_from_evil.clone(),
            Self::ShieldOfFaith => LANG_PACK.spells.shieldof_faith.clone(),
            Self::TurnUndead => LANG_PACK.spells.turn_undead.clone(),
            Self::Alarm => LANG_PACK.spells.alarm.clone(),
            Self::BurningHands => LANG_PACK.spells.burning_hands.clone(),
            Self::CharmPerson => LANG_PACK.spells.charm_person.clone(),
            Self::DetectMagic => LANG_PACK.spells.detect_magic.clone(),
            Self::FeatherFall => LANG_PACK.spells.feather_fall.clone(),
            Self::FloatingDisk => LANG_PACK.spells.floating_disk.clone(),
            Self::HoldPortal => LANG_PACK.spells.hold_portal.clone(),
            Self::MageArmor => LANG_PACK.spells.mage_armor.clone(),
            Self::MagicMissile => LANG_PACK.spells.magic_missile.clone(),
            Self::Sleep => LANG_PACK.spells.sleep.clone(),
        }
    }
}

impl Display for Spell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}: {}, {}, {}",
            self.to_feature(),
            LANG_PACK.level,
            self.level(),
            self.duration(),
            self.distance()
        )
    }
}

pub enum Distance {
    OnSelf,
    Close,
    Near,
    Far,
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            LANG_PACK.distance,
            match self {
                Self::OnSelf => LANG_PACK.distances.on_self.clone(),
                Self::Close => LANG_PACK.distances.close.clone(),
                Self::Near => LANG_PACK.distances.near.clone(),
                Self::Far => LANG_PACK.distances.far.clone(),
            }
        )
    }
}

pub enum Duration {
    Instant,
    Focus,
    Rounds(usize),
    Hours(usize),
    Days(usize),
    DaysDice(Dice),
}

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Instant => write!(f, "{}: {}", LANG_PACK.duration, LANG_PACK.durations.instant),
            Self::Focus => write!(f, "{}: {}", LANG_PACK.duration, LANG_PACK.durations.focus),
            Self::Rounds(count) => {
                write!(
                    f,
                    "{}: {} ({})",
                    LANG_PACK.duration, LANG_PACK.durations.rounds, count
                )
            }
            Self::Hours(count) => {
                write!(
                    f,
                    "{}: {} ({})",
                    LANG_PACK.duration, LANG_PACK.durations.hours, count
                )
            }
            Self::Days(count) => write!(
                f,
                "{}: {} ({})",
                LANG_PACK.duration, LANG_PACK.durations.days, count
            ),
            Self::DaysDice(dice) => write!(
                f,
                "{}: {} ({})",
                LANG_PACK.duration, LANG_PACK.durations.days_dice, dice
            ),
        }
    }
}
