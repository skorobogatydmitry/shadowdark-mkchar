# Shadowdark characted generation tool

Is inspired by https://github.com/cjstoddard/Shadowdark.py.

Features:
- randomly generate level 1 or level 0 character for Shadowdark.
- support translations (only russian for now)

# Build

`cargo build` works just fine for **Linux x86**.

For M1 from x86 Mac:
- `rustup target add aarch64-apple-darwin`
- `cargo build --release --target aarch64-apple-darwin`

> There's an extra step for MacOS users to disable security check.
> Otherwize, MacOS says the binary is corrupted and refuses to run it.
