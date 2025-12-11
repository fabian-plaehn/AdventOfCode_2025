use std::{
    clone,
    collections::{HashSet, VecDeque},
};

use mt_logger::*;
use utils::create_vector_from_tuple_string;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Button_Logic {
    diagram: Vec<bool>,
}

impl Button_Logic {
    fn add(&self, other: &Button_Logic) -> Button_Logic {
        assert!(self.diagram.len() == other.diagram.len());
        let mut result = Vec::new();
        for (&a, &b) in self.diagram.iter().zip(other.diagram.iter()) {
            result.push(a ^ b);
        }
        Button_Logic { diagram: result }
    }

    fn create_empty_button(size: usize) -> Button_Logic {
        Button_Logic {
            diagram: vec![false; size],
        }
    }

    fn equal(&self, other: &Button_Logic) -> bool {
        self.diagram
            .iter()
            .zip(other.diagram.iter())
            .into_iter()
            .all(|(a, b)| a == b)
    }

    fn from_vec(v: Vec<usize>, max_size: usize) -> Button_Logic {
        let mut result: Vec<bool> = Vec::new();
        for i in 0..max_size {
            if v.contains(&i) {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        Button_Logic { diagram: result }
    }

    fn from_indicator_string(s: &str) -> Button_Logic {
        let mut result: Vec<bool> = Vec::new();
        for c in s.chars() {
            if c == '.' {
                result.push(false);
            } else {
                result.push(true);
            }
        }
        Button_Logic { diagram: result }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Jolt_Logic {
    diagram: Vec<usize>,
}

impl Jolt_Logic {
    fn create_empty_button(size: usize) -> Jolt_Logic {
        Jolt_Logic {
            diagram: vec![0; size],
        }
    }

    fn add(&self, other: &Button_Logic) -> Jolt_Logic {
        let mut result = Vec::new();
        for (&a, &b) in self.diagram.iter().zip(other.diagram.iter()) {
            result.push(if b { a + 1 } else { a });
        }
        Jolt_Logic { diagram: result }
    }

    fn overshoot(&self, other: &Jolt_Logic) -> bool {
        self.diagram
            .iter()
            .zip(other.diagram.iter())
            .any(|(a, b)| a > b)
    }
}

#[derive(Debug)]
struct Machine {
    target_indicator_light: Button_Logic,
    button_wiring: Vec<Button_Logic>,
    joltage_requirements: Jolt_Logic,
}

impl Machine {
    fn from_string(s: &str) -> Machine {
        let mut target_indicator_light: Option<Button_Logic> = None;
        let mut size = 0;
        let mut button_wiring: Vec<Button_Logic> = Vec::new();
        let mut joltage_requirements: Option<Jolt_Logic> = None;
        for part in s.split(' ') {
            if part.chars().nth(0).unwrap() == '[' {
                target_indicator_light = Some(Button_Logic::from_indicator_string(
                    &part[1..part.len() - 1],
                ));
                size = target_indicator_light.as_ref().unwrap().diagram.len();
            } else if part.chars().nth(0).unwrap() == '(' {
                button_wiring.push(Button_Logic::from_vec(
                    create_vector_from_tuple_string(&part[1..part.len() - 1]),
                    size,
                ));
            } else if part.chars().nth(0).unwrap() == '{' {
                joltage_requirements = Some(Jolt_Logic {
                    diagram: create_vector_from_tuple_string(&part[1..part.len() - 1]),
                });
            }
        }
        Machine {
            target_indicator_light: target_indicator_light.unwrap(),
            button_wiring,
            joltage_requirements: joltage_requirements.unwrap(),
        }
    }
}

fn solve_button(
    target_indicator_light: Button_Logic,
    current_state: Button_Logic,
    button_wiring: Vec<Button_Logic>,
) -> usize {
    mt_log!(
        Level::Debug,
        "Solving {:?} to {:?}",
        current_state,
        target_indicator_light
    );
    mt_log!(Level::Debug, "Buttons_Available: {:?}", button_wiring);
    let mut queue: VecDeque<(Button_Logic, Vec<usize>)> = VecDeque::new();
    let mut visited: HashSet<Button_Logic> = HashSet::new();
    queue.push_back((current_state.clone(), Vec::new()));
    while let Some((state, seq)) = queue.pop_front() {
        //mt_log!(Level::Debug, "Current_State: {:?}, seq: {:?}", state, seq);
        if target_indicator_light.equal(&state) {
            mt_log!(Level::Debug, "Found shortest path: {:?}", seq);
            return seq.len();
        }

        for (i, button_logic) in button_wiring.iter().enumerate() {
            let new_state = state.add(button_logic);
            if !visited.insert(new_state.clone()) {
                continue;
            }
            let mut new_sequence = seq.clone();
            new_sequence.push(i);
            queue.push_back((new_state, new_sequence));
        }
    }
    0
}

fn solve_button_part_2_mf(
    target_indicator_light: Button_Logic,
    target_joult: Jolt_Logic,
    current_state: Button_Logic,
    button_wiring: Vec<Button_Logic>,
) -> usize {
    mt_log!(
        Level::Debug,
        "Solving {:?} to {:?}",
        current_state,
        target_indicator_light
    );
    mt_log!(Level::Debug, "Buttons_Available: {:?}", button_wiring);

    let jolt_state = Jolt_Logic::create_empty_button(target_joult.clone().diagram.len());
    let mut queue: VecDeque<(Button_Logic, Jolt_Logic, Vec<usize>)> = VecDeque::new();
    let mut visited: HashSet<(Button_Logic, Jolt_Logic)> = HashSet::new();
    queue.push_back((current_state.clone(), jolt_state.clone(), Vec::new()));
    while let Some((state, jolt_state, seq)) = queue.pop_front() {
        mt_log!(
            Level::Debug,
            "Current_State: {:?}, {:?}, seq: {:?}",
            state,
            jolt_state,
            seq
        );

        if target_joult.diagram == jolt_state.diagram {
            mt_log!(Level::Info, "Found shortest path: {:?}", seq);
            return seq.len();
        }

        for (i, button_logic) in button_wiring.iter().enumerate() {
            let new_state = state.add(button_logic);
            let new_jolt = jolt_state.add(button_logic);
            let mut new_sequence = seq.clone();
            new_sequence.push(i);
            if !visited.insert((new_state.clone(), new_jolt.clone())) {
                continue;
            }
            if new_jolt.overshoot(&target_joult) {
                //mt_log!(Level::Debug, "Overshooted: {:?}", new_jolt);
                continue;
            }

            queue.push_back((new_state, new_jolt, new_sequence));
        }
    }
    mt_flush!().unwrap();
    panic!("This should not happen");
}

fn button_to_indices(v: &[usize]) -> Vec<usize> {
    v.iter()
        .enumerate()
        .filter(|&(_, &bit)| bit == 1)
        .map(|(i, _)| i)
        .collect()
}

//use lp_modeler::problem::*;
use lp_modeler::dsl::*;
use lp_modeler::solvers::{CbcSolver, SolverTrait};
//use lp_modeler::variables::*;

fn minimize_button_sum(target: &[usize], buttons: &[Vec<usize>]) -> usize {
    let num_buttons = buttons.len();
    let num_elements = target.len();

    // Create LP problem
    let mut problem = LpProblem::new("minimize_button_sum", LpObjective::Minimize);

    // Create variables w_i (weights), assume integer non-negative
    let weights: Vec<LpInteger> = (0..num_buttons)
        .map(|i| LpInteger::new(&format!("w_{}", i)))
        .collect();

    // Set objective: minimize sum of w_i
    let objective = weights
        .iter()
        .fold(LpExpression::from(0.0), |acc, w| acc + w.clone());
    problem += objective;

    // Add constraints: sum_i w_i * buttons[i][j] == target[j]
    for j in 0..num_elements {
        let mut expr = LpExpression::from(0.0);
        for i in 0..num_buttons {
            expr = expr + weights[i].clone() * buttons[i][j] as f32;
        }
        problem += expr.equal(&LpExpression::from(target[j] as f32));
    }

    // Solve the problem using CBC solver
    let solver = CbcSolver::new();
    match solver.run(&problem) {
        Ok(solution) => {
            let mut sum = 0;
            for (name, value) in solution.results.iter() {
                println!("value of {} = {}", name, value);
            }
            sum
        }
        Err(_) => {
            // If no solution, return a sentinel value (or handle as needed)
            usize::MAX
        }
    }
}
#[derive(Debug)]
struct Machines {
    machines: Vec<Machine>,
}

impl Machines {
    fn from_string(s: &str) -> Machines {
        let mut machines = Vec::new();
        for line in s.lines() {
            machines.push(Machine::from_string(line));
        }
        Machines { machines }
    }
    #[allow(clippy::never_loop)]
    fn part_1(&self) {
        let mut count = 0;
        for machine in self.machines.iter() {
            count += solve_button(
                machine.target_indicator_light.clone(),
                Button_Logic::create_empty_button(machine.target_indicator_light.diagram.len()),
                machine.button_wiring.clone(),
            );
            mt_log!(Level::Info, "Result Part 1: {}", count);
        }
    }

    #[allow(clippy::never_loop)]
    fn part_2(&self) {
        let mut count = 0;
        for machine in self.machines.iter() {
            let buttons: Vec<Vec<usize>> = machine
                .button_wiring
                .iter()
                .map(|b| b.diagram.iter().map(|b| *b as usize).collect())
                .collect();
            let solution = minimize_button_sum(&machine.joltage_requirements.diagram, &buttons);
            mt_log!(Level::Info, "Found solution: {}", solution);
            count += solution;
        }
        mt_log!(Level::Info, "Result Part 2: {}", count);
    }
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
    let machines = Machines::from_string(&filecontent);

    //mt_log!(Level::Debug, "{:?}", machines);
    //machines.part_1();
    machines.part_2();
    mt_flush!().unwrap();
}
