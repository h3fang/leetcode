pub struct Spreadsheet {
    sheet: Vec<[i32; 26]>,
}

impl Spreadsheet {
    pub fn new(rows: i32) -> Self {
        Self {
            sheet: vec![[0; 26]; rows as usize + 1],
        }
    }

    fn index(cell: &str) -> (usize, usize) {
        let row: usize = cell[1..].parse().unwrap();
        let col = (cell.as_bytes()[0] - b'A') as usize;
        (row, col)
    }

    pub fn set_cell(&mut self, cell: String, value: i32) {
        let (row, col) = Self::index(&cell);
        self.sheet[row][col] = value;
    }

    pub fn reset_cell(&mut self, cell: String) {
        self.set_cell(cell, 0);
    }

    fn eval(&self, cell: &str) -> i32 {
        if cell.as_bytes()[0].is_ascii_uppercase() {
            let (row, col) = Self::index(cell);
            self.sheet[row][col]
        } else {
            cell.parse().unwrap()
        }
    }

    pub fn get_value(&self, formula: String) -> i32 {
        let (c1, c2) = formula[1..].split_once('+').unwrap();
        self.eval(c1) + self.eval(c2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut sheet = Spreadsheet::new(3);
        assert_eq!(12, sheet.get_value("=5+7".to_string()));
        sheet.set_cell("A1".to_string(), 10);
        assert_eq!(16, sheet.get_value("=A1+6".to_string()));
        sheet.set_cell("B2".to_string(), 15);
        assert_eq!(25, sheet.get_value("=A1+B2".to_string()));
        sheet.reset_cell("A1".to_string());
        assert_eq!(15, sheet.get_value("=A1+B2".to_string()));
    }
}
