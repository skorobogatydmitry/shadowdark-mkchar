use std::{cell::LazyCell, env, fs, path::Path};

use rand::seq::IndexedRandom;

use serde::Deserialize;

use crate::ancestry::Ancestry;

pub const LANG_PACK: LazyCell<LangPack> = LazyCell::new(|| LangPack::load());
static LANG_PACK_ENV_VAR_NAME: &str = "SHADOWDARK_MKCHAR_LANG_PACK_FILE";

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
    pub ancestries: Ancestries,
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
    pub talent: String,
    pub talents: Talents,
    pub deity: String,
    pub deities: Deities,
    pub background: String,
    pub backgrounds: Backgrounds,
    pub alignment: String,
    pub alignments: Alignments,
    pub inventory: String,
    pub purse: String,
    pub gold_pieces: String,
    pub silver_pieces: String,
    pub gear: Gear,
    pub name: String,
    pub names: Names,
    pub or: String,
}

impl LangPack {
    pub fn load() -> Self {
        let lang_file = env::var(LANG_PACK_ENV_VAR_NAME).unwrap_or(Self::default_lang_pack());
        serde_json::from_str(
            &fs::read_to_string(&lang_file)
                .expect(&format!("cannot read language file {}", lang_file)),
        )
        .expect(&format!("cannot read language file {}", lang_file))
    }

    // get relative to CWD or relative to binary
    fn default_lang_pack() -> String {
        let relative_to_cwd = "res/ru.json".to_string();
        if Path::new(&relative_to_cwd).exists() {
            relative_to_cwd
        } else {
            let basedir = match env::current_exe() {
                Ok(path) => {
                    if let Some(dir) = path.parent() {
                        dir.to_str().unwrap_or("").to_string()
                    } else {
                        "".to_string()
                    }
                }
                Err(_) => "".to_string(),
            };
            format!("{}/res/ru.json", basedir)
        }
    }
}

#[derive(Deserialize)]
pub struct ErrorMessages {
    pub stats_out_of_attempts: String,
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
    pub primordial: String,
}

#[derive(Deserialize)]
pub struct Ancestries {
    pub dwarf: String,
    pub kobold: String,
    pub elf: String,
    pub goblin: String,
    pub halfling: String,
    pub half_orc: String,
    pub human: String,
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

#[derive(Deserialize)]
pub struct Talents {
    pub weapon_mastery: String,
    pub precise_strike: String,
    pub trained: String,
    pub armor_training: String,
    pub gifted: String,
    pub versatile: String,
    pub vigilant: String,
    pub deadly_stab: String,
    pub god_blessed: String,
    pub spell_expert: String,
    pub skilled_caster: String,
    pub devoted: String,
    pub thin_air_craft: String,
    pub bookworm: String,
}

#[derive(Deserialize)]
pub struct Deities {
    pub saint_terragnis: String,
    pub gede: String,
    pub madeera_the_covenant: String,
    pub ord: String,
    pub memnon: String,
    pub ramlaat: String,
    pub shune_the_vile: String,
    pub the_lost: String,
}

#[derive(Deserialize)]
pub struct Backgrounds {
    pub urchin: String,
    pub wanted: String,
    pub cult_initiate: String,
    pub thieves_guild: String,
    pub banished: String,
    pub orphaned: String,
    pub wizards_apprentice: String,
    pub jeweler: String,
    pub herbalist: String,
    pub barbarian: String,
    pub mercenary: String,
    pub sailor: String,
    pub alcolyte: String,
    pub soldier: String,
    pub ranger: String,
    pub scout: String,
    pub minstrel: String,
    pub scholar: String,
    pub nobel: String,
    pub chirurgeon: String,
}

#[derive(Deserialize)]
pub struct Alignments {
    pub chaotic: String,
    pub neutral: String,
    pub lawful: String,
}

#[derive(Deserialize)]
pub struct Gear {
    pub torch: String,
    pub dagger: String,
    pub pole: String,
    pub shortbow: String,
    pub arrows: String,
    pub rope: String,
    pub flask_of_oil: String,
    pub crowbar: String,
    pub iron_spikes: String,
    pub flint_and_steel: String,
    pub grappling_hook: String,
    pub club: String,
    pub bag_of_caltrops: String,
}

#[derive(Deserialize)]
pub struct Names {
    pub dwarf: Vec<String>,
    pub elf: Vec<String>,
    pub goblin: Vec<String>,
    pub halfling: Vec<String>,
    pub half_orc: Vec<String>,
    pub human: Vec<String>,
    pub kobold: Vec<String>,
}

impl Names {
    pub fn roll(&self, ancestry: &Ancestry) -> String {
        let mut rng = rand::rng();
        match ancestry {
            Ancestry::Dwarf => self.dwarf.choose(&mut rng),
            Ancestry::Elf => self.elf.choose(&mut rng),
            Ancestry::Goblin => self.goblin.choose(&mut rng),
            Ancestry::Halfling => self.halfling.choose(&mut rng),
            Ancestry::HalfOrc => self.half_orc.choose(&mut rng),
            Ancestry::Human => self.human.choose(&mut rng),
            Ancestry::Kobold => self.kobold.choose(&mut rng),
        }
        .unwrap()
        .clone()
    }
}
