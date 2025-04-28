use shadowdark_mkchar::Character;
use shadowdark_mkchar::args::ARGS;
use shadowdark_mkchar::template::ToPdf;

fn main() {
    let character = Character::new();
    if !ARGS.no_print {
        println!("{}", character);
    }
    if !ARGS.no_pdf {
        character.to_pdf();
    }
}
