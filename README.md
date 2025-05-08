# Shadowdark characted generation tool

Is inspired by https://github.com/cjstoddard/Shadowdark.py.

Features:
- randomly generate level 1 or level 0 character for Shadowdark
- supports [translations](#translations) (only russian is built)
- prints character to console and makes a PDF file

# HOWTO use the tool ?

- you need to download [one of the release archives from github](https://github.com/skorobogatydmitry/shadowdark-mkchar/releases/latest) depending on your OS and architecture
- unpack the archive: `shadowdark-mkchar-...` folder should appear
- then open a terminal window
  > You could just click on the `shadowdark-mkchar` file in the folder
- then run something like `~/Downloads/shadowdark-mkchar-*/shadowdark-mkchar`
  > MacOS may say that the file is corrupted and can't be launched. It's because the tool is not signed through Apple. You need to manually force MacOS to trust it: `sudo xattr -r -d com.apple.quarantine ~/Downloads/shadowdark-mkchar-*/shadowdark-mkchar`

Explore what args are supported by `~/Downloads/shadowdark-mkchar-*/shadowdark-mkchar --help`.

## Translations

The tool supports language packs.
Run the tool with `--help` to see what's the default translation used by the tool.
The tool searches for the file in its directory and the working directory.
`--translation` argument could be used to pick the file like this: `./shadowdark-mkchar --translation lang/alt.json`

# PDF

The tool uses a [typst](https://github.com/typst/typst) template to generate PDF.
`cargo install --locked typst-cli` is required to compile template manually.
Then you need to uncomment `// #import "inputs-sample.typ": inputs` in the [res/template.typ](res/template.typ) and comment the import below.
PDF compilation command: `typst compile ./res/template.typ sample.pdf --font-path ./res/`.

# Build

## Makefile / automated

There's a [Makefile](./Makefile) to make release archives. It only works on Linux for now.

## Manual

### `cargo build` works just fine for **Linux x86**.
### Linux ARM
```sh
sudo pacman -S extra/aarch64-linux-gnu-gcc
cargo install cross
cross build --target aarch64-unknown-linux-gnu
```

### For M1 from x86 Mac
```
rustup target add aarch64-apple-darwin
cargo build --target aarch64-apple-darwin
```

> There's an extra step for MacOS users to disable security check.
> Otherwize, MacOS says the binary is corrupted and refuses to run it.

### Windows

See [Makefile](./Makefile)'s `release-win` target. It relies on cygwin + rustup setup with msvc.

# What to contribute?

## Usability & new features

- other language packs
- randomize class in case there are several equal stats (e.g. if STR and INT are 16, choose either warrior or wizard)
- ancestry talent rolls (e.g. human's extra one)
- re-roll duplicated talents when applicable (e.g. theif's vigilant)
- have meta spell lists and an arg to use the meta or random
- save character to YAML/ JSON + have interactive editor mode for e.g. spell list

## Refactoring

- cleanup values clonning in the code
- change the typst template to dynamically adjust layout by checking actual content size
