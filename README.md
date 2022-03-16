# Connect Four

Simple graphical implementation of the Connect Four board game, in Rust with [ggez](//ggez.rs).  

`ggez` requires that the font (`iosevka.ttf`) be placed in `target/debug/resources` or `target/release/resources` (depending on build).

# Build Instructions

Assuming an installed [Rust toolchain](//rustup.rs):

```bash
cd c4rs
cargo build --release
```

Use `cargo run --release` instead to run as well.
