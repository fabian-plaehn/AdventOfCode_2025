use mt_logger::*;

fn main() {
    mt_new!(None, Level::Info, OutputStream::StdOut, true);
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let content: String = std::fs::read_to_string(filename).expect("Error reading file");
    let content: Vec<&str> = content.lines().collect();

    let mut total_joltage = 0;
    for string_number in content {
        total_joltage += find_largest_n_number(string_number, 12);
    }
    mt_log!(Level::Info, "{}", total_joltage);
    mt_flush!().unwrap();
}
fn find_largest_n_number(string_number: &str, n: usize) -> u128 {
    assert!(string_number.len() >= n);
    // find biggest number and second biggest number which is right of biggest number
    let mut numbers: Vec<Option<u128>> = vec![None; n];
    // initiate Vec

    let mut current_number: u128;
    let length = string_number.len();
    mt_log!(
        Level::Debug,
        "trying to find largest number in: {}",
        string_number
    );
    for (i, c) in string_number.chars().enumerate() {
        current_number = c.to_digit(10).unwrap().into();
        mt_log!(Level::Debug, "current_number is {}", current_number);
        let mut new_numbers = numbers.clone();
        let mut replaced_flag = false;
        for (j, number) in numbers.iter().enumerate() {
            mt_log!(
                Level::Debug,
                "RF: {}, numbers: {:?}, new_numbers: {:?}, main_condi: {}",
                replaced_flag,
                numbers,
                new_numbers,
                (length - i) >= (n - j)
            );
            if replaced_flag {
                new_numbers[j] = None;
            } else if (length - i) >= (n - j)
                && (number.is_none() || number.unwrap() < current_number)
            {
                mt_log!(Level::Debug, "found new highest number for position {}", j);
                new_numbers[j] = Some(current_number);
                replaced_flag = true;
            }
        }
        numbers = new_numbers;
    }
    mt_log!(Level::Debug, "Numbers: {:?}", numbers);

    let sum = numbers
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| n.unwrap() * 10u128.pow(i as u32))
        .sum::<u128>();
    mt_log!(Level::Debug, "SUM: {}", sum);
    sum
}
