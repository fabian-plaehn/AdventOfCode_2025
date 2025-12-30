use mt_logger::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Shape {
    cells: [bool; 9], // 3x3, row-major
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shapes: Vec<usize>,
}

#[derive(Debug)]
struct Puzzle {
    shapes: HashMap<usize, Shape>,
    regions: Vec<Region>,
}

impl Puzzle {
    fn part_1(&self) {
        let result = self
            .regions
            .iter()
            .filter(|region| {
                region.shapes.iter().sum::<usize>() * 9 <= region.width * region.height
            })
            .count();
        mt_log!(Level::Info, "Result Part 1: {}", result);
    }
}

fn parse_puzzle(input: &str) -> Puzzle {
    let mut shapes = HashMap::new();
    let mut regions = Vec::new();

    let mut lines = input.lines().peekable();

    while let Some(line) = lines.peek() {
        let line = line.trim();

        // Skip empty lines
        if line.is_empty() {
            lines.next();
            continue;
        }

        // ---------- Shape ----------
        if line.ends_with(':') && !line.contains('x') {
            let id: usize = line[..line.len() - 1].parse().expect("Invalid shape id");

            lines.next(); // consume "id:"

            let mut cells = [false; 9];

            for row in 0..3 {
                let row_line = lines.next().expect("Missing shape row").trim();

                assert!(
                    row_line.len() == 3,
                    "Shape row must be exactly 3 characters"
                );

                for (col, ch) in row_line.chars().enumerate() {
                    cells[row * 3 + col] = ch == '#';
                }
            }

            shapes.insert(id, Shape { cells });
            continue;
        }

        // ---------- Region ----------
        if line.contains('x') && line.contains(':') {
            let line = lines.next().unwrap();
            let (dim_part, data_part) = line.split_once(':').expect("Invalid region line");

            let (w, h) = dim_part.split_once('x').expect("Invalid region dimensions");

            let width: usize = w.trim().parse().unwrap();
            let height: usize = h.trim().parse().unwrap();

            let shapes_vec: Vec<usize> = data_part
                .split_whitespace()
                .map(|v| v.parse().expect("Invalid shape reference"))
                .collect();

            regions.push(Region {
                width,
                height,
                shapes: shapes_vec,
            });
            continue;
        }

        panic!("Unrecognized line: {}", line);
    }

    Puzzle { shapes, regions }
}

fn main() {
    mt_new!(None, Level::Debug, OutputStream::StdOut, true);

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        mt_log!(Level::Error, "Usage: {} <input.txt>", args[0]);
        mt_flush!().unwrap();
        std::process::exit(1);
    }

    let filepath = &args[1];
    let filecontent = std::fs::read_to_string(filepath).expect("Could not read file");

    let puzzle = parse_puzzle(&filecontent);

    mt_log!(Level::Info, "Parsed puzzle:\n{:#?}", puzzle);
    puzzle.part_1();
    mt_flush!().unwrap();
}
