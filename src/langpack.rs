use std::{cell::LazyCell, fs};

use serde::Deserialize;

use crate::ancestry::Ancestry;

pub const PACK: LazyCell<LangPack> = LazyCell::new(|| LangPack::load());

#[derive(Deserialize)]
pub struct LangPack {
    pub error_messages: ErrorMessages,
    pub stats: String,
    pub strength: String,
    pub dexterity: String,
    pub constitution: String,
    pub intellegence: String,
    pub wisdom: String,
    pub charisma: String,
    pub ancestry: String,
    pub ancestries: Vec<Ancestry>,
    pub language: String,
    pub languages: Languages,
    pub ancestry_features: AncestryFeatures,
    pub class: String,
    pub class_args: ClassArgs,
    pub hit_points: String,
    pub weapon: String,
    pub weapon_masteries: WeaponMasteries,
    pub armor: String,
    pub armor_masteries: ArmorMasteries,
    pub features: String,
    pub class_features: ClassFeatures,
}

impl LangPack {
    pub fn load() -> Self {
        // TODO: read env and pick a correct lang pack
        let file_name = "res/ru.json";
        serde_json::from_str(&fs::read_to_string(file_name).expect("cannot read language file"))
            .expect("cannot read language pack file")
    }
}

#[derive(Deserialize)]
pub struct ErrorMessages {
    pub stats_out_of_attempts: String,
    pub not_all_ancestries: String,
    pub unknown_class_option: String,
}

#[derive(Deserialize)]
pub struct Languages {
    pub common: String,
    pub dwarwish: String,
    pub elvish: String,
    pub sylvan: String,
    pub goblin: String,
    pub orchish: String,
    pub draconic: String,
    pub giant: String,
    pub merran: String,
    pub reptillian: String,
    pub thanian: String,
    pub celestial: String,
    pub diabolic: String,
    pub primodial: String,
}

#[derive(Deserialize)]
pub struct AncestryFeatures {
    pub stout: Feature,
    pub farsight: Feature,
    pub keen_senses: Feature,
    pub mighty: Feature,
    pub stealthy: Feature,
    pub ambitious: Feature,
    pub knack: Feature,
}

#[derive(Deserialize, Clone)]
pub struct Feature {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ClassArgs {
    pub zero: String,
    pub any: String,
    pub fighter: String,
    pub thief: String,
    pub wizard: String,
    pub priest: String,
}

#[derive(Deserialize)]
pub struct WeaponMasteries {
    pub all: String,
    pub club: String,
    pub crossbow: String,
    pub mace: String,
    pub longsword: String,
    pub staff: String,
    pub warhammer: String,
    pub dagger: String,
    pub shortbow: String,
    pub shortsword: String,
}

#[derive(Deserialize)]
pub struct ArmorMasteries {
    pub all: String,
    pub shields: String,
    pub leather_armor: String,
    pub mithral_chainmail: String,
}

#[derive(Deserialize)]
pub struct ClassFeatures {
    pub hauler: Feature,
    pub weapon_mastery: Feature,
    pub grit: Feature,
    pub turn_undead: Feature,
    pub spellcasting: Feature,
    pub learning_spells: Feature,
    pub backstab: Feature,
    pub theivery: Feature,
    pub jack_of_all_trades: Feature,
    pub beginners_luck: Feature,
}
