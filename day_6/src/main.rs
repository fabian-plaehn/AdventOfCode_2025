use core::num;

use mt_logger::*;
use utils::filter_non_relevant_chars;

#[derive(Clone, Debug)]
enum Operator {
    Multiplication,
    Addition,
}

#[derive(Debug)]
struct MathProblem {
    tasks: Vec<(Vec<i128>, Operator)>,
}

impl MathProblem {
    fn from_str_part1(s: &str) -> MathProblem {
        let num_lines: usize = s.lines().collect::<Vec<&str>>().len();
        let mut tasks: Vec<(Vec<i128>, Operator)> = Vec::new();

        for (i, line) in s.lines().enumerate() {
            mt_log!(Level::Debug, "Original Line: {:?}", line);
            let filtered_line = filter_non_relevant_chars(line);
            mt_log!(Level::Debug, "Filtered Line: {:?}", filtered_line);
            if i == 0 {
                tasks = vec![(Vec::new(), Operator::Addition); filtered_line.len()];
            }
            if i == num_lines - 1 {
                let operators: Vec<Operator> = filtered_line
                    .iter()
                    .map(|number_str| match number_str.trim() {
                        "*" => Operator::Multiplication,
                        _ => Operator::Addition,
                    })
                    .collect();
                for (j, operator) in operators.iter().enumerate() {
                    tasks[j].1 = operator.clone();
                }
            } else {
                let numbers: Vec<i128> = filtered_line
                    .iter()
                    .map(|number_str| {
                        number_str
                            .trim()
                            .parse::<i128>()
                            .unwrap_or_else(|_| panic!("could not parse: {}", number_str))
                    })
                    .collect();
                for (j, number) in numbers.iter().enumerate() {
                    tasks[j].0.push(*number);
                }
            }
        }
        mt_log!(Level::Debug, "{:?}", tasks);
        MathProblem { tasks }
    }
    fn from_str_part2(s: &str) -> MathProblem {
        let lines: Vec<&str> = s.lines().collect();
        let mut tasks: Vec<(Vec<i128>, Operator)> = Vec::new();
        let mut numbers: Vec<i128> = Vec::new();
        let mut number_str: Vec<char> = Vec::new();
        for i in (0..lines[0].chars().collect::<Vec<char>>().len()).rev() {
            for line in &lines {
                let c = line.chars().nth(i).unwrap();
                match c {
                    ' ' => {
                        if !number_str.is_empty() {
                            numbers.push(
                                number_str
                                    .into_iter()
                                    .collect::<String>()
                                    .parse()
                                    .expect("couldnt parse"),
                            );
                            number_str = Vec::new();
                        }
                    }
                    '*' => {
                        if !number_str.is_empty() {
                            numbers.push(
                                number_str
                                    .into_iter()
                                    .collect::<String>()
                                    .parse()
                                    .expect("couldnt parse"),
                            );
                            number_str = Vec::new();
                        }
                        tasks.push((numbers, Operator::Multiplication));
                        numbers = Vec::new();
                    }
                    '+' => {
                        if !number_str.is_empty() {
                            numbers.push(
                                number_str
                                    .into_iter()
                                    .collect::<String>()
                                    .parse()
                                    .expect("couldnt parse"),
                            );
                            number_str = Vec::new();
                        }
                        tasks.push((numbers, Operator::Addition));
                        numbers = Vec::new();
                    }
                    _ => {
                        number_str.push(c);
                    }
                }
            }
        }
        MathProblem { tasks }
    }
    fn solve(&self) -> i128 {
        let mut total_sum = 0;
        for task in &self.tasks {
            total_sum += match task.1 {
                Operator::Addition => {
                    let mut result = 0;
                    for number in &task.0 {
                        result += number;
                    }
                    result
                }
                Operator::Multiplication => {
                    let mut result = 1;
                    for number in &task.0 {
                        result *= number;
                    }
                    result
                }
            }
        }
        total_sum
    }
}

fn main() {
    mt_new!(None, Level::Debug, OutputStream::StdOut, true);
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        mt_log!(Level::Error, "Usage: {} <inputfile>", args[0]);
        mt_flush!().unwrap();
        std::process::exit(1);
    }
    let filename = &args[1];
    let file_content = std::fs::read_to_string(filename).expect("Could not read file");
    let math_problem = MathProblem::from_str_part1(&file_content);
    mt_log!(Level::Info, "Result Part 1: {}", math_problem.solve());
    let math_problem = MathProblem::from_str_part2(&file_content);
    mt_log!(Level::Debug, "{:?}", math_problem);
    mt_flush!().unwrap();
    mt_log!(Level::Info, "Result Part 2: {}", math_problem.solve());
    mt_flush!().unwrap();
}
