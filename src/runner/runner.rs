use crate::sudoku;
use crate::tree;

#[derive(Debug)]
pub struct Runner {}

impl Runner {
    pub fn new() -> Runner {
        Runner {}
    }

    pub fn run(&self) {
        let sudoku = sudoku::Sudoku::new();
        let tree = tree::Tree::new(10);
        println!("Sudoku: {:?}", sudoku.to_string());
        println!("Dummy tree, {:?}", tree);
    }
}