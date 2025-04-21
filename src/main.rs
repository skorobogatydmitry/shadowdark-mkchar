use clap::Parser;

fn main() {
    shadowdark_mkchar::make_character(shadowdark_mkchar::Args::parse());
}
