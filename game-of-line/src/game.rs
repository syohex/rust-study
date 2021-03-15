use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    rows: usize,
    columns: usize,
    cells: Vec<Vec<Cell>>,
}

impl Universe {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            cells: vec![vec![Cell::Dead; columns]; rows],
        }
    }

    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for (row, col) in cells.iter() {
            self.cells[*row][*col] = Cell::Alive;
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.rows {
            for col in 0..self.columns {
                let lives = self.live_neighbor_count(row, col);

                next[row][col] = match (self.cells[row][col], lives) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (other, _) => other,
                };
            }
        }

        self.cells = next;
    }

    pub fn live_neighbor_count(&self, row: usize, column: usize) -> i32 {
        let mut count = 0;
        for &row_delta in &[-1, 0, 1] {
            for &col_delta in &[-1, 0, 1] {
                if row_delta == 0 && col_delta == 0 {
                    continue;
                }

                let r = (self.rows + row) as i32;
                let r = r + row_delta;
                let r = r as usize % self.rows;

                let c = (self.columns + column) as i32;
                let c = c + col_delta;
                let c = c as usize % self.columns;

                count += match self.cells[r][c] {
                    Cell::Alive => 1,
                    Cell::Dead => 0,
                };
            }
        }

        count
    }

    pub fn row_as_string(&self, row: usize) -> Option<String> {
        if row < self.rows {
            let mut row_string = String::new();
            for &cell in &self.cells[row] {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                row_string.push(symbol);
            }

            Some(row_string)
        } else {
            None
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.columns {
                let symbol = if self.cells[row][col] == Cell::Dead {
                    '◻'
                } else {
                    '◼'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
