use std::{
    collections::{HashMap, HashSet},
    thread::current,
};

use mt_logger::*;
use utils::Point3D;
struct JunctionField {
    positions: Vec<Point3D>,
    connected_pairs: HashSet<(Point3D, Point3D)>,
    num_boxes: usize,
}

impl JunctionField {
    fn from_string(s: &str) -> Self {
        let positions = s
            .lines()
            .map(|line| {
                let nums: Vec<f64> = line
                    .split(',')
                    .map(|n| n.parse::<f64>().expect("Could not parse number"))
                    .collect();
                Point3D {
                    position: (nums[0], nums[1], nums[2]),
                }
            })
            .collect();
        JunctionField {
            positions,
            connected_pairs: HashSet::new(),
            num_boxes: s.lines().count(),
        }
    }

    fn get_closest_boxes(&self) -> Vec<(Point3D, Point3D)> {
        let mut pairs = Vec::new();
        let pts = &self.positions;

        for i in 0..pts.len() {
            for j in (i + 1)..pts.len() {
                pairs.push((pts[i], pts[j])); // if Point3D is Copy
            }
        }

        pairs.sort_by(|pairA, pairB| {
            pairA
                .0
                .distance(&pairA.1)
                .total_cmp(&pairB.0.distance(&pairB.1))
        });
        mt_log!(Level::Debug, "{:?}", pairs);
        pairs
    }

    fn part_1(&self) {
        let mut circuits: Vec<Vec<Point3D>> = Vec::new();
        let pairs = self.get_closest_boxes();

        for pair in pairs.iter().take(1000) {
            mt_log!(Level::Debug, "{:?}", pair);
            //mt_log!(Level::Debug, "{:?}", circuits);
            // check if any of the points are in any circuit
            if let Some(index_a) = check_if_in_circuit(&pair.0, &circuits) {
                if let Some(index_b) = check_if_in_circuit(&pair.1, &circuits) {
                    mt_log!(Level::Debug, "Found circuit where pointA and pointB are in");
                    if index_a != index_b {
                        merge_circuit(&mut circuits, index_a, index_b);

                        if circuits[0].len() == self.num_boxes {
                            mt_log!(Level::Debug, "All Junction have been merged");
                            break;
                        }
                    }
                    continue;
                }
                circuits[index_a].push(pair.1);
                if circuits[0].len() == self.num_boxes {
                    mt_log!(Level::Debug, "All Junction have been merged");
                    break;
                }
                continue;
            }

            if let Some(index_a) = check_if_in_circuit(&pair.1, &circuits) {
                if let Some(index_b) = check_if_in_circuit(&pair.0, &circuits) {
                    if index_a != index_b {
                        merge_circuit(&mut circuits, index_a, index_b);
                        if circuits[0].len() == self.num_boxes {
                            mt_log!(Level::Debug, "All Junction have been merged");
                            break;
                        }
                    }
                    continue;
                }
                circuits[index_a].push(pair.0);
                if circuits[0].len() == self.num_boxes {
                    mt_log!(Level::Debug, "All Junction have been merged");
                    break;
                }
                continue;
            }

            circuits.push(vec![pair.0, pair.1]);
        }
        let mut circuits_lens = circuits.iter().map(|c| c.len()).collect::<Vec<usize>>();
        circuits_lens.sort();
        circuits_lens.reverse();
        let mut result = 1;
        for circuit_len in circuits_lens.iter().take(3) {
            result *= circuit_len;
        }
        mt_log!(Level::Debug, "{:?}", circuits_lens);
        mt_log!(Level::Info, "Result Part 1: {}", result);
    }

    fn part_2(&self) {
        let mut circuits: Vec<Vec<Point3D>> = Vec::new();
        let pairs = self.get_closest_boxes();
        let mut result_part_2 = 0.0;
        for pair in pairs.iter() {
            mt_log!(Level::Debug, "{:?}", pair);
            //mt_log!(Level::Debug, "{:?}", circuits);
            // check if any of the points are in any circuit
            if let Some(index_a) = check_if_in_circuit(&pair.0, &circuits) {
                if let Some(index_b) = check_if_in_circuit(&pair.1, &circuits) {
                    mt_log!(Level::Debug, "Found circuit where pointA and pointB are in");
                    if index_a != index_b {
                        merge_circuit(&mut circuits, index_a, index_b);

                        if circuits[0].len() == self.num_boxes {
                            result_part_2 = pair.0.position.0 * pair.1.position.0;
                            mt_log!(Level::Debug, "All Junction have been merged");
                            break;
                        }
                    }
                    continue;
                }
                circuits[index_a].push(pair.1);
                if circuits[0].len() == self.num_boxes {
                    result_part_2 = pair.0.position.0 * pair.1.position.0;
                    mt_log!(Level::Debug, "All Junction have been merged");
                    break;
                }
                continue;
            }

            if let Some(index_a) = check_if_in_circuit(&pair.1, &circuits) {
                if let Some(index_b) = check_if_in_circuit(&pair.0, &circuits) {
                    if index_a != index_b {
                        merge_circuit(&mut circuits, index_a, index_b);
                        if circuits[0].len() == self.num_boxes {
                            result_part_2 = pair.0.position.0 * pair.1.position.0;
                            mt_log!(Level::Debug, "All Junction have been merged");
                            break;
                        }
                    }
                    continue;
                }
                circuits[index_a].push(pair.0);
                if circuits[0].len() == self.num_boxes {
                    result_part_2 = pair.0.position.0 * pair.1.position.0;
                    mt_log!(Level::Debug, "All Junction have been merged");
                    break;
                }
                continue;
            }

            circuits.push(vec![pair.0, pair.1]);
        }
        let mut circuits_lens = circuits.iter().map(|c| c.len()).collect::<Vec<usize>>();
        circuits_lens.sort();
        circuits_lens.reverse();
        let mut result = 1;
        for circuit_len in circuits_lens.iter().take(3) {
            result *= circuit_len;
        }
        mt_log!(Level::Debug, "{:?}", circuits_lens);

        mt_log!(Level::Info, "Result Part 2: {}", result_part_2);
    }
}
fn check_if_in_circuit(p: &Point3D, c: &[Vec<Point3D>]) -> Option<usize> {
    for (i, circuit) in c.iter().enumerate() {
        if circuit.contains(p) {
            return Some(i);
        }
    }
    None
}

fn merge_circuit(c: &mut Vec<Vec<Point3D>>, index_a: usize, index_b: usize) {
    let circuit = c[index_a].clone();
    for point in circuit {
        c[index_b].push(point);
    }
    c.remove(index_a);
}
fn main() {
    mt_new!(None, Level::Info, OutputStream::StdOut, true);
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        mt_log!(Level::Error, "Usage: {} <input.txt>", args[0]);
    }
    let filepath = &args[1];
    let filecontent = std::fs::read_to_string(filepath).expect("Could not read file");
    let junction_field = JunctionField::from_string(&filecontent);
    junction_field.part_1();
    junction_field.part_2();
    mt_flush!().unwrap();
}
