use std::{cell::LazyCell, env, fmt::Display, fs, path::Path};

use derive_typst_intoval::{IntoDict, IntoValue};
use typst::foundations::IntoValue;

use rand::seq::IndexedRandom;

use serde::Deserialize;

use crate::{ancestry::Ancestry, args::ARGS};

pub const LANG_PACK: LazyCell<LangPack> = LazyCell::new(|| LangPack::load());

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
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
    pub available: String,
    pub ft: String,
    pub spell: String,
    pub spells: Spells,
    pub level: String,
    pub distance: String,
    pub distances: Distances,
    pub duration: String,
    pub durations: Durations,
    pub dice: String,
}

impl LangPack {
    pub fn load() -> Self {
        let lang_file = Self::find_lang_pack();
        serde_json::from_str(
            &fs::read_to_string(&lang_file)
                .expect(&format!("cannot read language file {}", lang_file)),
        )
        .expect(&format!("cannot read language file {}", lang_file))
    }

    // get relative to CWD or relative to binary
    fn find_lang_pack() -> String {
        let relative_to_cwd = &ARGS.translation;
        if Path::new(&relative_to_cwd).exists() {
            relative_to_cwd.clone()
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
            format!("{}/{}", basedir, relative_to_cwd)
        }
    }
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
pub struct ErrorMessages {
    pub stats_out_of_attempts: String,
    pub non_common_language: String,
    pub pdf_compilation_failed: String,
    pub incorrect_spell_class: String,
    pub incorrect_spells_count: String,
    pub not_enough_spells: String,
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
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

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
pub struct Ancestries {
    pub dwarf: String,
    pub kobold: String,
    pub elf: String,
    pub goblin: String,
    pub halfling: String,
    pub half_orc: String,
    pub human: String,
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
pub struct AncestryFeatures {
    pub stout: Feature,
    pub farsight: Feature,
    pub keen_senses: Feature,
    pub mighty: Feature,
    pub stealthy: Feature,
    pub ambitious: Feature,
    pub knack: Feature,
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
pub struct Feature {
    pub name: String,
    pub description: String,
}

impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
pub struct ClassArgs {
    pub zero: String,
    pub fighter: String,
    pub thief: String,
    pub wizard: String,
    pub priest: String,
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
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

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
pub struct ArmorMasteries {
    pub all: String,
    pub shields: String,
    pub leather_armor: String,
    pub mithral_chainmail: String,
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
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

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
pub struct Talents {
    pub weapon_mastery: Feature,
    pub precise_strike: Feature,
    pub trained: Feature,
    pub armor_training: Feature,
    pub gifted: Feature,
    pub versatile: Feature,
    pub vigilant: Feature,
    pub deadly_stab: Feature,
    pub god_blessed: Feature,
    pub spell_expert: Feature,
    pub skilled_caster: Feature,
    pub devoted: Feature,
    pub thin_air_craft: Feature,
    pub bookworm: Feature,
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
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

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
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

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
pub struct Alignments {
    pub chaotic: String,
    pub neutral: String,
    pub lawful: String,
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
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

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
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
    pub fn roll(&self, ancestry: &Ancestry) -> &String {
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
    }
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
pub struct Spells {
    pub cure_wounds: Feature,
    pub holy_weapon: Feature,
    pub light: Feature,
    pub protection_from_evil: Feature,
    pub shieldof_faith: Feature,
    pub turn_undead: Feature,
    pub alarm: Feature,
    pub burning_hands: Feature,
    pub charm_person: Feature,
    pub detect_magic: Feature,
    pub feather_fall: Feature,
    pub floating_disk: Feature,
    pub hold_portal: Feature,
    pub mage_armor: Feature,
    pub magic_missile: Feature,
    pub sleep: Feature,
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
pub struct Distances {
    pub on_self: String,
    pub close: String,
    pub near: String,
    pub far: String,
}

#[derive(Deserialize, Clone, Debug, IntoValue, IntoDict)]
pub struct Durations {
    pub instant: String,
    pub focus: String,
    pub rounds: String,
    pub hours: String,
    pub days: String,
    pub days_dice: String,
}
