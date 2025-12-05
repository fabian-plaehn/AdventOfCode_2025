use std::collections::HashSet;

use mt_logger::*;

struct IngredientDB {
    ranges: Vec<(u128, u128)>,
}
impl IngredientDB {
    fn from_string(s: &str) -> IngredientDB {
        IngredientDB {
            ranges: s
                .lines()
                .map(|line| {
                    let mut parts = line.split("-");
                    let a = parts.next().unwrap().parse::<u128>().unwrap();
                    let b = parts.next().unwrap().parse::<u128>().unwrap();
                    (a, b)
                })
                .collect(),
        }
    }

    fn id_exists(&self, id: u128) -> bool {
        for &(a, b) in &self.ranges {
            if a <= id && id <= b {
                return true;
            }
        }
        false
    }

    fn num_fresh_ids(&self) -> u128 {
        let mut sorted_ranges: Vec<(u128, u128)> = self.ranges.clone();
        sorted_ranges.sort();

        let mut filtered_ranges: Vec<(u128, u128)>;
        loop {
            filtered_ranges = Vec::new();
            let mut merges = false;
            let mut skip = false;
            for i in 0..sorted_ranges.len() - 1 {
                let (a, b) = sorted_ranges[i];
                let (a2, b2) = sorted_ranges[i + 1];
                if skip {
                    skip = false;
                    continue;
                }
                mt_log!(Level::Debug, "{},{}, {},{}", a, b, a2, b2);
                if b < a2 {
                    filtered_ranges.push((a, b));
                    if i == (sorted_ranges.len() - 2) {
                        filtered_ranges.push((a2, b2));
                    }
                    continue;
                }
                merges = true;
                mt_log!(Level::Debug, "MERGE HAPPENING");
                if b >= b2 {
                    filtered_ranges.push((a, b));
                    skip = true;
                } else {
                    filtered_ranges.push((a, b2));
                }
            }
            if !merges {
                break;
            }
            sorted_ranges = filtered_ranges.clone();
            mt_log!(Level::Debug, "after 1 iteration: {:?}", filtered_ranges);
            mt_log!(Level::Info, "len: {}", filtered_ranges.len());
        }
        mt_log!(
            Level::Debug,
            "final: {:?}, {}",
            filtered_ranges,
            filtered_ranges.len()
        );

        let mut num_ids_fresh = 0;
        for (a, b) in filtered_ranges {
            num_ids_fresh += b - a + 1;
        }
        num_ids_fresh
    }
}

fn main() {
    mt_new!(None, Level::Debug, OutputStream::StdOut, true);
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        mt_log!(Level::Error, "Usage: {} <input>", args[0]);
        mt_flush!().unwrap();
        std::process::exit(1);
    }
    let filename = &args[1];
    let file_content = std::fs::read_to_string(filename).expect("Couldnt read file");
    let ingredient_db_string: Vec<&str> = file_content.split("\r\n\r\n").collect();
    mt_log!(Level::Debug, "{:?}", ingredient_db_string);
    let ingredient_db = IngredientDB::from_string(ingredient_db_string.first().unwrap());

    let mut count = 0;
    for id_str in ingredient_db_string.get(1).unwrap().lines() {
        if ingredient_db.id_exists(id_str.parse::<u128>().unwrap()) {
            count += 1;
        }
    }
    mt_log!(Level::Info, "Result Part 1: {}", count);
    mt_log!(
        Level::Info,
        "Result Part 2: {}",
        ingredient_db.num_fresh_ids()
    );
    mt_flush!().unwrap();
}
