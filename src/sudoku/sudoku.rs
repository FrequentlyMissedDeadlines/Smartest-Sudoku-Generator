use itertools::Itertools;

pub struct Sudoku {
    pub values: [ [u8 ; 9] ; 9]
}

impl Sudoku {
    pub fn new() -> Sudoku { 
        Sudoku {
            values: [ [0 ; 9] ; 9]
        }
    }

    pub fn copy(&self) -> Sudoku {
        let mut values = [ [0 ; 9] ; 9];
        for x in 0..9 {
            for y in 0..9 {
                values[x][y] = self.values[x][y];
            }
        }
        Sudoku {
            values: values
        }
    }

    pub fn parse(string: String) -> Sudoku {
        let mut values = [ [0 ; 9] ; 9];
        for x in 0..9 {
            for y in 0..9 {
                values[x][y] = (string.chars().nth(x * 9 + y).unwrap() as u8) - ('0' as u8);
            }
        }
        Sudoku {
            values: values
        }
    }

    pub fn to_string(&self) -> String {
        self.values.to_vec().iter().map(|a| a.to_vec().iter().join("")).join("")
    }

    pub fn is_solved(&self) -> bool {
        let expected = 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;
        for x in 0..9 {
            let mut col = 1;
            let mut row = 1;
            let mut square = 1;
            for y in 0..9 {
                row *= self.values[x][y] as i32;
                col *= self.values[y][x] as i32;
                square *= self.values[3 * (x % 3) + y % 3][3 * (x / 3) + y / 3] as i32;
            }
            if row != expected {
                return false;
            } else if col != expected {
                return false;
            } else if square != expected {
                return false;
            }


        }
        
        return true;
    }

    pub fn new_solved() -> Sudoku {
        let mut values = [ [0 ; 9] ; 9];
        for x in 0..9 {
            values[0][x] = (x + 1) as u8;
        }

        Sudoku {
            values: values
        }
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

    #[test]
    fn test_is_solved() {
        let empty = Sudoku {
            values: [ [0 ; 9] ; 9]
        };
        let solved = Sudoku::parse(String::from("534678912672195348198342567859761423426853791713924856961537284287419635345286179"));
        let wrong_square = Sudoku::parse(String::from("123456789200000000300000000400000000500000000600000000700000000800000000900000000"));
        let wrong_row = Sudoku::parse(String::from("100000000200000000300000000400000000500000000600000000700000000800000000900000000"));
        let wrong_col = Sudoku::parse(String::from("123456789200000000300000000400000000500000000600000000700000000800000000000000000"));

        assert_eq!(empty.is_solved(), false);
        assert_eq!(wrong_square.is_solved(), false);
        assert_eq!(wrong_row.is_solved(), false);
        assert_eq!(wrong_col.is_solved(), false);

        assert_eq!(solved.is_solved(), true);
    }

    #[test]
    fn test_parse() {
        let string = "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
        let tested = Sudoku::parse(String::from(string));

        assert_eq!(tested.to_string(), string);
    }

    #[test]
    fn test_copy() {
        let string = "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
        let tested = Sudoku::parse(String::from(string)).copy();

        assert_eq!(tested.to_string(), string);
    }
}