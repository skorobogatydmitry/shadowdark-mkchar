use clap::Parser;
use shadowdark_mkchar::Character;

fn main() {
    let character = Character::new(shadowdark_mkchar::Args::parse());
    println!("{}", character);
}
