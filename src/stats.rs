use std::{collections::BTreeMap, fmt::Display};

use crate::{Dice, translation::LANG_PACK};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum StatKind {
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
            StatKind::Strength => LANG_PACK.strength.clone(),
            StatKind::Dexterity => LANG_PACK.dexterity.clone(),
            StatKind::Constitution => LANG_PACK.constitution.clone(),
            StatKind::Intellegence => LANG_PACK.intellegence.clone(),
            StatKind::Wisdom => LANG_PACK.wisdom.clone(),
            StatKind::Charisma => LANG_PACK.charisma.clone(),
        }
    }
}

impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:15}: {:2} ({:2})",
            &self.name(),
            self.modifier(),
            self.val
        )
    }
}

#[derive(Debug)]
pub struct Stats {
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
            panic!("{}", LANG_PACK.error_messages.stats_out_of_attempts);
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
            "{}:\n  {}",
            LANG_PACK.stats,
            self.map
                .values()
                .map(|a| format!("{a}"))
                .collect::<Vec<String>>()
                .join("\n  ")
        )
    }
}
