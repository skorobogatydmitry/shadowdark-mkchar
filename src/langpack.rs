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
