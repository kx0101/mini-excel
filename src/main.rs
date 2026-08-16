use std::collections::HashMap;

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
    InvalidRow,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct CellKey {
    column: String,
    row: usize,
}

impl CellKey {
    fn parse(input: &str) -> Result<CellKey, ParseError> {
        let first_index = input
            .char_indices()
            .find(|(_, c)| c.is_ascii_digit())
            .map(|(i, _)| i);

        let Some(first_index) = first_index else {
            return Err(ParseError::InvalidFormat);
        };

        let column = &input[..first_index];
        let row = &input[first_index..];

        if column.is_empty() || !column.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(ParseError::InvalidFormat);
        }

        if row.is_empty() || !row.chars().all(|c| c.is_ascii_digit()) {
            return Err(ParseError::InvalidFormat);
        }

        let row = row.parse::<usize>().map_err(|_| ParseError::InvalidRow)?;
        if row == 0 {
            return Err(ParseError::InvalidRow);
        }

        Ok(CellKey {
            column: column.to_uppercase(),
            row,
        })
    }
}

#[derive(Debug)]
enum CellValue {
    Number(f64),
    Boolean(bool),
    Text(String),
}

#[derive(Debug)]
struct Cell {
    raw_input: String,
    value: CellValue,
}

struct Spreadsheet {
    cells: HashMap<CellKey, Cell>,
}

impl Spreadsheet {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    fn set(&mut self, key: CellKey, value: Cell) -> Option<Cell> {
        self.cells.insert(key, value)
    }

    fn get(&self, key: &CellKey) -> Option<&Cell> {
        self.cells.get(key)
    }
}

fn main() {
    let mut spreadsheet = Spreadsheet::new();

    let cell_a = CellKey {
        column: String::from("AA"),
        row: 10,
    };
    let cell_b = CellKey {
        column: String::from("B"),
        row: 2,
    };
    let value = CellValue::Number(5.0);
    let cell = Cell {
        raw_input: String::from("5.0"),
        value,
    };

    spreadsheet.set(cell_a.clone(), cell);

    match spreadsheet.get(&cell_a) {
        Some(value) => println!("{:?}", value),
        None => println!("not found"),
    }

    match spreadsheet.get(&cell_b) {
        Some(value) => println!("{:?}", value),
        None => println!("not found"),
    }

    let result = dbg!(CellKey::parse("aa10"));
    let result = dbg!(CellKey::parse("12A"));
    let result = dbg!(CellKey::parse("A"));
    let result = dbg!(CellKey::parse("A0"));
    let result = dbg!(CellKey::parse("A1B"));
}
