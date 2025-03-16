use std::collections::HashMap;
use std::str::FromStr;

struct Spreadsheet {
    cells: HashMap<String, i32>,
}

impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.cells.insert(cell, value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.set_cell(cell, 0);
    }

    fn get_value(&self, mut formula: String) -> i32 {
        formula.remove(0);
        let mut arr: Vec<&str> = formula.split("+").collect();
        self.value(arr[0]) + self.value(arr[1])
    }

    fn value(&self, s: &str) -> i32 {
        FromStr::from_str(s).unwrap_or_else(|_| *self.cells.get(s).unwrap_or(&0))
    }
}
