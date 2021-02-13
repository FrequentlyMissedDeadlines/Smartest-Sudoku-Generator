mod runner;
mod sudoku;
mod tree;

fn main() {
    runner::Runner::new().run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}