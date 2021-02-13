use itertools::Itertools;

pub struct Sudoku {
    pub values: [ [i8 ; 9] ; 9]
}

impl Sudoku {
    pub fn new() -> Sudoku { 
        Sudoku {
            values: [ [0 ; 9] ; 9]
        }
    }

    pub fn to_string(&self) -> String {
        self.values.to_vec().iter().map(|a| a.to_vec().iter().join("")).join("")
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_to_string() {
        let tested = Sudoku::new();
        assert_eq!(tested.to_string(), "0".repeat(81));
    }

    #[test]
    fn test_struct_def() {
        let tested = Sudoku {
            values: [ [0 ; 9] ; 9]
        };
        assert_eq!(tested.to_string(), "0".repeat(81));
    }
}