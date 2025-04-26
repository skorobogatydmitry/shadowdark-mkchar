# Shadowdark characted generation tool

Is inspired by https://github.com/cjstoddard/Shadowdark.py.

Features:
- randomly generate level 1 or level 0 character for Shadowdark.
- support translations (only russian for now)

# Run

- you need to download and unpack one of the release archives from github
- then, open terminal window
- and run something like `./Downloads/shadowdark-mkchar-*/shadowdark-mkchar`

## Translations

The tool supports language packs.  
`lang/ru.json` is the default translation supplied with the tool.
The tool searches for it near itself and in working directory.  
`SHADOWDARK_MKCHAR_LANG_PACK_FILE` environment variable could be used to overwrite it like this: `SHADOWDARK_MKCHAR_LANG_PACK_FILE=./lang/alt.json ./shadowdark-mkchar`

# Build

## Manual

### `cargo build` works just fine for **Linux x86**.
### Linux ARM
```sh
sudo pacman -S extra/aarch64-linux-gnu-gcc
cargo install cross
cross build --target aarch64-unknown-linux-gnu
```

### For M1 from x86 Mac
- `rustup target add aarch64-apple-darwin`
- `cargo build --target aarch64-apple-darwin`

> There's an extra step for MacOS users to disable security check.
> Otherwize, MacOS says the binary is corrupted and refuses to run it.

# NEXT

- print to a PDF/HTML template
- randomize class for several eqaual stats
- have a flag for truly random class
- re-roll duplicated talents when applicable (theif, vigilant)
- fix `res` search path