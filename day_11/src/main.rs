use std::collections::{HashMap, HashSet, VecDeque};

use mt_logger::*;

struct Node {
    id: usize,
    connections: Vec<usize>,
}

struct Network {
    nodes: Vec<Node>,
}

fn parse_graph(s: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    for line in s.lines() {
        let (node, connections) = line.split_once(':').unwrap();
        let node = node.trim().to_string();
        mt_log!(Level::Debug, "{:?}", connections);
        let edges: Vec<String> = connections
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        graph.insert(node, edges);
    }
    graph
}

fn find_all_path(graph: HashMap<String, Vec<String>>, start: String, end: &String) {
    let mut queue: VecDeque<(Vec<String>, HashSet<String>)> = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(start.clone());
    queue.push_back((vec![start], visited));

    let mut paths: Vec<Vec<String>> = Vec::new();

    while let Some((path, visited_set)) = queue.pop_front() {
        for possible_next_state in graph[path.last().unwrap()].iter() {
            if visited_set.contains(possible_next_state) {
                continue;
            }
            let mut new_visited_set: HashSet<String> = visited_set.clone();
            let mut new_path = path.clone();
            if !new_visited_set.insert(possible_next_state.clone()) {
                continue;
            }
            new_path.push(possible_next_state.clone());
            if new_path.last().unwrap() == end {
                paths.push(new_path);
                continue;
            }

            queue.push_back((new_path, new_visited_set));
        }
    }
    mt_log!(Level::Debug, "Paths: {:?}", paths);
    mt_log!(Level::Info, "Solution Part 1: {}", paths.len());
}

fn dfs(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    end: &str,
    visited: &mut HashSet<String>,
    path: &mut Vec<String>,
    results: &mut Vec<Vec<String>>,
) {
    visited.insert(current.to_string());
    path.push(current.to_string());

    if current == end {
        results.push(path.clone()); // only clone final paths
    } else if let Some(neighbors) = graph.get(current) {
        for next in neighbors {
            if !visited.contains(next) {
                dfs(graph, next, end, visited, path, results);
            }
        }
    }

    // backtrack:
    path.pop();
    visited.remove(current);
}

fn find_all_paths(
    graph: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
) -> Vec<Vec<String>> {
    let mut results = Vec::new();
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    dfs(graph, start, end, &mut visited, &mut path, &mut results);

    results
}

fn reverse_graph(graph: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut rev = HashMap::new();
    for (a, bs) in graph {
        for b in bs {
            rev.entry(b.clone())
                .or_insert_with(Vec::new)
                .push(a.clone());
        }
    }
    rev
}

fn reachable_to(rev: &HashMap<String, Vec<String>>, target: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(target.to_string());
    set.insert(target.to_string());

    while let Some(n) = queue.pop_front() {
        if let Some(parents) = rev.get(&n) {
            for p in parents {
                if set.insert(p.clone()) {
                    queue.push_back(p.clone());
                }
            }
        }
    }

    set
}

// count paths in a pruned graph: DAG toward target
fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    node: &str,
    target: &str,
    reachable: &HashSet<String>,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if node == target {
        return 1;
    }
    if let Some(&v) = memo.get(node) {
        return v;
    }
    let mut sum = 0u64;
    if let Some(neigh) = graph.get(node) {
        for n in neigh {
            if reachable.contains(n) {
                sum += count_paths(graph, n, target, reachable, memo);
            }
        }
    }
    memo.insert(node.to_string(), sum);
    sum
}

pub fn count_paths_with_required(
    graph: &HashMap<String, Vec<String>>,
    start: &str,
    fft: &str,
    dac: &str,
    out: &str,
) -> u64 {
    let rev = reverse_graph(graph);

    // segment reachability
    let r_fft = reachable_to(&rev, fft);
    let r_dac = reachable_to(&rev, dac);
    let r_out = reachable_to(&rev, out);

    // Option A: start → fft → dac → out
    let c1 = count_paths(graph, start, fft, &r_fft, &mut HashMap::new());
    let c2 = count_paths(graph, fft, dac, &r_dac, &mut HashMap::new());
    let c3 = count_paths(graph, dac, out, &r_out, &mut HashMap::new());
    let a = c1 * c2 * c3;

    // Option B: start → dac → fft → out
    let c4 = count_paths(graph, start, dac, &r_dac, &mut HashMap::new());
    let c5 = count_paths(graph, dac, fft, &r_fft, &mut HashMap::new());
    let c6 = count_paths(graph, fft, out, &r_out, &mut HashMap::new());
    let b = c4 * c5 * c6;

    a + b
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
    let graph = parse_graph(&filecontent);
    mt_log!(Level::Debug, "Graph: {:?}", graph);
    let results = find_all_paths(&graph, "you", "out");
    mt_log!(Level::Info, "Result Part 1: {}", results.len());

    let results = count_paths_with_required(&graph, "svr", "fft", "dac", "out");
    mt_log!(Level::Info, "Result Part 1: {}", results);
    mt_flush!().unwrap();
}
