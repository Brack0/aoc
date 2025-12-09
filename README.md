# Advent of Code

## TLDR

- In Rust
- Details in README.md

## Frequently used commands

Dev mode

```sh
cargo watch -x check -x clippy -x test -x run
```

Debug performance

```sh
cargo flamegraph --dev
```

Production mode

```sh
cargo build --release
hyperfine --warmup 3 [binary]
```
