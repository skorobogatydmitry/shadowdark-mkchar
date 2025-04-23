# Shadowdark characted generation tool

Is inspired by https://github.com/cjstoddard/Shadowdark.py.

Features:
- randomly generate level 1 or level 0 character for Shadowdark.
- support translations (only russian for now)

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