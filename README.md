# Smartest Sudoku Generator
![Build](https://github.com/FrequentlyMissedDeadlines/Smartest-Sudoku-Generator/workflows/Build/badge.svg?branch=main)

This sudoku generator is smart enough to detect the difficulty of the generated sudoku.

## Build and run
``` bash
# Run in debug mode
cargo run

# Run optimized release version
cargo run --release

# Run unit tests
cargo test

# Get unit tests coverage report
rustup component add llvm-tools-preview
cargo install grcov
export RUSTC_BOOTSTRAP=1
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
cargo clean
cargo test
grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/ --ignore src/main.rs
```