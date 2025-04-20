use std::{cell::LazyCell, fs};

use serde::Deserialize;

pub const PACK: LazyCell<LangPack> = LazyCell::new(|| LangPack::load());

#[derive(Deserialize)]
pub struct LangPack {
    pub stats_out_of_attempts: String,
    pub stats: String,
    pub strength: String,
    pub dexterity: String,
    pub constitution: String,
    pub intellegence: String,
    pub wisdom: String,
    pub charisma: String,
}

impl LangPack {
    pub fn load() -> Self {
        // TODO: read env and pick a correct lang pack
        let file_name = "res/ru.json";
        serde_json::from_str(&fs::read_to_string(file_name).expect("cannot read language file"))
            .expect("cannot read language pack file")
    }
}
