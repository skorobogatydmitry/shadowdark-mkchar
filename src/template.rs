use derive_typst_intoval::{IntoDict, IntoValue};
use std::fs;
use typst::foundations::{Dict, IntoValue};
use typst_as_lib::TypstEngine;

use crate::{
    Character,
    args::ARGS,
    class::ClassFeature,
    translation::{Feature, LANG_PACK, LangPack},
};

const TYPST_TEMPLATE: &str = include_str!("../res/template.typ");
const FONT1: &[u8] = include_bytes!("../res/Brahms Gotisch Cyrillic/BrahmsGotischCyr.otf");
const FONT2: &[u8] = include_bytes!("../res/Kereru/Kereru Bold.ttf");

#[derive(Debug, Clone, IntoValue, IntoDict)]
struct TypstTemplateData {
    terms: LangPack,
    name: String,
    languages: Vec<String>,
    background: String,
    ancestry: String,
    hit_points: u32,
    stats: Vec<Stat>,
    alignment: String,
    deity: String,
    ancestry_feature: Feature,
    class_features: Vec<Feature>,
    class: String,
    weapon_masteries: Vec<String>,
    armor_masteries: Vec<String>,
    gold_pieces: u32,
    silver_pieces: u32,
    slots_count: u8,
    items: Vec<String>,
    spells: Vec<Feature>,
}

impl From<TypstTemplateData> for Dict {
    fn from(value: TypstTemplateData) -> Self {
        value.into_dict()
    }
}

impl From<&Character> for TypstTemplateData {
    fn from(value: &Character) -> Self {
        Self {
            terms: LANG_PACK.clone(),
            name: value.name.clone(),
            background: value.background.name(),
            languages: value.languages().iter().map(|l| format!("{l}")).collect(),
            ancestry: value.ancestry_attributes.ancestry.name(),
            ancestry_feature: value.ancestry_attributes.ancestry.feature().to_feature(),
            hit_points: value.max_hit_points(),
            stats: value
                .stats
                .iter()
                .map(|s| Stat {
                    name: s.name(),
                    value: s.val,
                    modifier: s.modifier(),
                })
                .collect(),
            alignment: value.alignment.name(),
            deity: value.deity.name(),
            class_features: value
                .class_attributes
                .class_features
                .iter()
                .map(|f| f.to_feature())
                .collect(),
            class: value.class_attributes.class_name(),
            weapon_masteries: value
                .class_attributes
                .weapon_masteries
                .iter()
                .map(|wm| wm.name())
                .collect(),
            armor_masteries: value
                .class_attributes
                .armor_masteries
                .iter()
                .map(|am| am.name())
                .collect(),
            gold_pieces: value.inventory.purse.gold,
            silver_pieces: value.inventory.purse.silver,
            slots_count: value.inventory_slots_count(),
            items: value
                .inventory
                .equipment
                .iter()
                .map(|i| i.to_string())
                .collect(),
            spells: value
                .class_attributes
                .class_features
                .iter()
                .find_map(|f| {
                    if let ClassFeature::SpellCasting(spells) = f {
                        Some(spells.iter().map(|s| s.to_feature()).collect())
                    } else {
                        None
                    }
                })
                .unwrap_or(vec![]),
        }
    }
}

#[derive(Debug, Clone, IntoValue, IntoDict)]
struct Stat {
    name: String,
    value: u8,
    modifier: i8,
}

pub trait ToPdf {
    fn to_pdf(&self);
}

impl ToPdf for Character {
    fn to_pdf(&self) {
        let file_name = format!(
            "{}-{}.pdf",
            self.name.to_lowercase(),
            self.class_attributes.class_name().to_lowercase()
        );
        let template = ARGS
            .template
            .clone()
            .map(|tf| fs::read_to_string(tf).unwrap());
        let engine = TypstEngine::builder()
            .main_file(match template.as_ref() {
                None => TYPST_TEMPLATE,
                Some(t) => t,
            })
            .fonts([FONT1, FONT2])
            .build();
        let data = TypstTemplateData::from(self);
        let doc = engine
            .compile_with_input(data)
            .output
            .map_err(|e| format!("{}: {e}", LANG_PACK.error_messages.pdf_compilation_failed))
            .unwrap();

        let pdf = typst_pdf::pdf(&doc, &Default::default())
            .map_err(|e| format!("{}: {e:?}", LANG_PACK.error_messages.pdf_compilation_failed))
            .unwrap();

        fs::write(file_name, pdf)
            .map_err(|e| format!("{}: {e:?}", LANG_PACK.error_messages.pdf_compilation_failed))
            .unwrap();
    }
}
