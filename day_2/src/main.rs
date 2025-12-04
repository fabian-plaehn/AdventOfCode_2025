#[derive(Debug)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn from_string(s: &str) -> Vec<Interval> {
        s.split(',')
            .map(|range| {
                let range = range.split('-').collect::<Vec<&str>>();
                Interval {
                    start: range[0]
                        .parse::<i64>()
                        .unwrap_or_else(|_| panic!("Error parsing {}", range[0])),
                    end: range[1]
                        .parse::<i64>()
                        .unwrap_or_else(|_| panic!("Error parsing {}", range[1])),
                }
            })
            .collect::<Vec<Interval>>()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let content: String = std::fs::read_to_string(filename).expect("Error reading file");
    let intervals = Interval::from_string(&content);
    let mut count: i64 = 0;
    for interval in intervals.iter() {
        count += find_invalid_ids(interval);
    }
    println!("{}", count);
}

fn find_invalid_ids(interval: &Interval) -> i64 {
    let mut count: i64 = 0;
    for i in interval.start..interval.end + 1 {
        // put 2 for part 1 and usize::MAX-1 for part 2
        if is_invalid_id(i, usize::MAX - 5) {
            count += i;
        }
    }
    count
}

fn is_invalid_id(id: i64, repeating_maximum: usize) -> bool {
    let id_str = id.to_string();
    for i in 1..(id_str.len() / 2 + 1) {
        let chars = &id_str[0..i];
        let splits = id_str.split(chars).collect::<Vec<&str>>();
        if splits.iter().map(|s| s.len()).sum::<usize>() == 0
            && splits.len() < (repeating_maximum + 1)
        {
            return true;
        }
    }
    false
}
