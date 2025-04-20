use std::{collections::BTreeMap, fmt::Display};

use rand::Rng;

mod langpack;

enum Dice {
    D6 = 6,
}

impl Dice {
    fn roll(&self) -> u8 {
        let max = match self {
            Self::D6 => 6,
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
            panic!("{}", langpack::PACK.stats_out_of_attempts);
        }

        let map = attrs.into_iter().map(|a| (a.kind.clone(), a)).collect();
        Self { map }
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

pub fn make_character() {
    let stats = Stats::generate();
    println!("{} | {stats}", langpack::PACK.stats);
}
