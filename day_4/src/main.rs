use std::collections::btree_map::Range;

use mt_logger::*;

#[derive(Debug)]
struct Field(Vec<Vec<char>>);

impl Field {
    fn from_string(s: &str) -> Field {
        Field(
            s.to_string()
                .lines()
                .map(|line| line.chars().collect())
                .collect(),
        )
    }
    fn to_string(field: Field) -> String {
        let mut s: String = String::new();
        for row in field.0 {
            for c in row {
                s.push(c);
            }
            s.push('\n');
        }
        s
    }
}

fn main() {
    mt_new!(None, Level::Debug, OutputStream::StdOut, true);
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        mt_log!(Level::Error, "Usage: {} <filename>", args[0]);
        mt_flush!().unwrap();
        std::process::exit(1);
    }
    let filename = &args[1];
    let filecontent = std::fs::read_to_string(filename).expect("Error reading file");
    // parse string
    let field: Vec<Vec<char>> = Field::from_string(&filecontent).0;
    let result = task_1(field, '@');
    mt_log!(Level::Info, "Result: {}", result);
    mt_flush!().unwrap();
}

fn get_adjacent_indexes(
    index: (usize, usize),
    max_row: isize,
    max_col: isize,
) -> Vec<(usize, usize)> {
    let (row, col) = index;
    let mut a_indexes: Vec<(usize, usize)> = Vec::new();
    for i in -1isize..2 {
        for j in -1isize..2 {
            let new_row: isize = row as isize + i;
            let new_col: isize = col as isize + j;
            if !(0 <= new_row && new_row < max_row && 0 <= new_col && new_col < max_col)
                || (new_row == row as isize && new_col == col as isize)
            {
                continue;
            }
            a_indexes.push((new_row as usize, new_col as usize));
        }
    }
    a_indexes
}

fn find_number_neighbours(
    field: &Vec<Vec<char>>,
    object: char,
    index: (usize, usize),
    max_row: isize,
    max_col: isize,
) -> i32 {
    let (row, col) = index;
    let mut count = 0;
    // loop through neighbouhrs
    let a_index: Vec<(usize, usize)> = get_adjacent_indexes((row, col), max_row, max_col);
    mt_log!(
        Level::Debug,
        "Index {},{} has a_index: {:?}",
        row,
        col,
        a_index
    );
    for (a_i, a_j) in a_index {
        if field[a_i][a_j] == object {
            count += 1;
        }
    }
    count
}

fn task_1(field: Vec<Vec<char>>, object: char) -> i32 {
    let mut count = 0;
    let max_row = field.len();
    let max_col = field[0].len();
    let mut result_field = field.clone();
    for (i, line) in field.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != object {
                continue;
            }
            if find_number_neighbours(&field, object, (i, j), max_row as isize, max_col as isize)
                < 4
            {
                result_field[i][j] = 'x';
                mt_log!(
                    Level::Debug,
                    "Index: {},{} has less than 4 neighbours",
                    i,
                    j
                );
                count += 1;
            }
        }
    }
    std::fs::write("output.txt", Field::to_string(Field(result_field)))
        .expect("Error writing file");
    count
}
