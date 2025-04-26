use clap::Parser;
use shadowdark_mkchar::Character;

fn main() {
    let character = Character::new(shadowdark_mkchar::args::Args::parse());
    println!("{}", character);
}
