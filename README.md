# Shadowdark characted generation tool

Is inspired by https://github.com/cjstoddard/Shadowdark.py.

Features:
- randomly generate level 1 or level 0 character for Shadowdark
- support [translations](#translations) (only russian for now)
- prints character to console and makes a PDF file

# Run

- you need to download and unpack one of the release archives from github
- then open terminal window
- then run something like `./Downloads/shadowdark-mkchar-*/shadowdark-mkchar`

## Translations

The tool supports language packs.  
Run the tool with `--help` to see what's the default translation supplied with the tool.  
The tool searches for the file in its directory and in the working directory.  
`--translation` argument could be used to overwrite the file like this: `./shadowdark-mkchar --translation lang/alt.json`

# PDF

The tool uses a [typst](https://github.com/typst/typst) template to generate PDF.  
`cargo install --locked typst-cli` is required to compile template manually.  
Then you need to uncomment `// #import "inputs-sample.typ": inputs` in the [res/template.typ](res/template.typ) and comment the import below.  
PDF compilation command: `typst compile ./res/template.typ sample.pdf --font-path ./res/`.

# Build

## Makefile / automated

There's a makefile to make release archives. It only works on Linux for now.

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

- allow to choose spells
- fix dashes in skills
- add spells description in a separated block
- randomize class for several eqaual stats
- re-roll duplicated talents when applicable (theif, vigilant)
- talent rolls (e.g. human's extra one)
- deity as argument
- cleanup values clonning in the code
