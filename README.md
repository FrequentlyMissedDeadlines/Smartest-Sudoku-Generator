# Smartest Sudoku Generator
![Build](https://github.com/FrequentlyMissedDeadlines/Smartest-Sudoku-Generator/workflows/Build/badge.svg?branch=main)
![Release](https://github.com/FrequentlyMissedDeadlines/Smartest-Sudoku-Generator/workflows/Release/badge.svg?branch=main)
[![codecov](https://codecov.io/gh/FrequentlyMissedDeadlines/Smartest-Sudoku-Generator/branch/main/graph/badge.svg?token=GBZ6Z36YDM)](https://codecov.io/gh/FrequentlyMissedDeadlines/Smartest-Sudoku-Generator)
[![](https://tokei.rs/b1/github/FrequentlyMissedDeadlines/Smartest-Sudoku-Generator?category=code)](https://github.com/FrequentlyMissedDeadlines/Smartest-Sudoku-Generator)

This sudoku generator is smart enough to detect the difficulty of the generated sudoku.

## Usage

WIP, not ready yet but this repo might not stay public as it seems that good algorithms to generate sudokus can make a lot of money in mobile apps. Star repo to keep it public.

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
grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
```