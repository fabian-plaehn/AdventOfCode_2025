// Start 50
// min 0, max 99
// R8 -> +8
// L8 -> -8
// 0 + L1 -> 99

// Password: num of dial points visited at 0

use core::num;
use std::fs;

use utils::{get_rotations, get_rotations_number, map_number};

#[derive(PartialEq)]
enum Method {
    Standard,
    Advanced,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let method: Method;
    if args.len() < 3 {
        method = Method::Standard;
    } else {
        match args[2].as_str() {
            "standard" => method = Method::Standard,
            "0x434C49434B" => method = Method::Advanced,
            _ => {
                println!("Invalid method");
                std::process::exit(1);
            }
        }
    }

    let content: String = fs::read_to_string(filename).expect("Error reading file");
    let content: Vec<&str> = content.lines().collect();

    let mut pos = 50;
    let mut password = 0;
    for line in content.iter() {
        let first_character = line.chars().next();
        let mut number = line[1..].parse::<i32>().unwrap();
        match first_character {
            Some('L') => number = -number,
            Some('R') => {}
            _ => {
                panic!("Invalid input");
            }
        }
        // map pos back to 0 to 99
        let mut advanced_method_str = "".to_string();
        if method == Method::Advanced {
            let rotations = get_rotations(pos, number, 0, 99);
            if (pos + number) > 99 && (pos + number) % 100 != 0 {
                assert!(rotations > 0, "{} {} {}", pos, number, rotations);
            }
            if rotations > 0 {
                password += rotations;
                advanced_method_str =
                    format!(" during this rotation it points to 0 {} times", rotations).to_string();
            }
        }
        pos += number;
        pos = map_number(pos, 0, 99);
        if pos == 0 {
            password += 1;
        }

        println!(
            "The dial is rotated {} to point at {} {}",
            line, pos, advanced_method_str
        );
    }
    println!("Password: {}", password);
}

// 6106
