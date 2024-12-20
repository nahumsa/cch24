use std::fmt;

const ROWS: usize = 5;
const COLUMNS: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Blank,
    Wall,
    Cookies,
    Milk,
}

#[derive(Clone)]
pub struct Board {
    pub state: [[Cell; COLUMNS]; ROWS],
}

impl Board {
    pub fn new() -> Self {
        let state = [[Cell::Blank; COLUMNS]; ROWS];
        Self { state }
    }

    pub fn reset(&mut self) {
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                if column == 0 || column == COLUMNS - 1 || row == ROWS - 1 {
                    self.set_cell(row, column, Cell::Wall);
                }
            }
        }
    }

    pub fn set_cell(&mut self, row: usize, column: usize, cell_type: Cell) {
        if row < ROWS && column < COLUMNS {
            self.state[row][column] = cell_type;
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.state.iter().enumerate() {
            for &cell in row.iter() {
                let symbol = match cell {
                    Cell::Blank => "⬛",   // Blank
                    Cell::Cookies => "🍪", // Cookies
                    Cell::Milk => "🥛",    // Milk
                    Cell::Wall => "⬜",    // Wall
                };

                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_initialization() {
        let board = Board::new();
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                assert_eq!(
                    board.state[row][column],
                    Cell::Blank,
                    "Cell at ({}, {}) should be Blank",
                    row,
                    column
                );
            }
        }
    }

    #[test]
    fn test_set_cell() {
        let mut board = Board::new();
        let row = 2;
        let column = 3;
        let cell = Cell::Cookies;
        board.set_cell(row, column, cell);

        assert_eq!(
            board.state[row][column], cell,
            "Cell at ({}, {}) should be Cookies",
            row, column
        );
    }

    #[test]
    fn test_set_cell_out_of_bounds() {
        let mut board = Board::new();
        // These should not panic or modify any cell
        board.set_cell(ROWS, COLUMNS, Cell::Milk);
        board.set_cell(100, 100, Cell::Wall);

        // Ensure the board is still initialized with default values
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                assert_eq!(
                    board.state[row][column],
                    Cell::Blank,
                    "Cell at ({:?}, {:?}) should remain Blank",
                    row,
                    column
                );
            }
        }
    }

    #[test]
    fn test_reset_display() {
        let mut board = Board::new();
        board.reset();

        let expected_display = "\
⬜⬛⬛⬛⬛⬜
⬜⬛⬛⬛⬛⬜
⬜⬛⬛⬛⬛⬜
⬜⬛⬛⬛⬛⬜
⬜⬜⬜⬜⬜⬜
";

        assert_eq!(
            format!("{}", board),
            expected_display,
            "Board display did not match expected output"
        );
    }
}
