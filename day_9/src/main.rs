use indicatif::{ProgressBar, ProgressStyle};

use mt_logger::*;
use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point(isize, isize);

impl Point {
    fn size_rectangle(&self, other: &Point) -> usize {
        (self.0.abs_diff(other.0) + 1) * (self.1.abs_diff(other.1) + 1)
    }

    fn between(&self, other: &Point) -> Vec<Point> {
        let (x1, x2) = (self.0.min(other.0), self.0.max(other.0));
        let (y1, y2) = (self.1.min(other.1), self.1.max(other.1));

        let mut result = Vec::new();
        for x in x1..x2 + 1 {
            for y in y1..y2 + 1 {
                if (x == x1 && y == y1) || (x == x2 && y == y2) {
                    continue;
                }
                result.push(Point(x, y));
            }
        }
        result
    }

    fn add(self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }

    fn in_bounds(&self, top_left: (isize, isize), bottom_right: (isize, isize)) -> bool {
        if top_left.0 <= self.0
            && self.0 <= bottom_right.0
            && top_left.1 <= self.1
            && self.1 <= bottom_right.1
        {
            return true;
        }
        false
    }

    fn in_bounds_strict(&self, top_left: (isize, isize), bottom_right: (isize, isize)) -> bool {
        if top_left.0 < self.0
            && self.0 < bottom_right.0
            && top_left.1 < self.1
            && self.1 < bottom_right.1
        {
            return true;
        }
        false
    }

    fn rect_in_boundset(&self, other: &Point, boundset: &HashSet<Point>) -> bool {
        let (x1, x2) = (self.0.min(other.0), self.0.max(other.0));
        let (y1, y2) = (self.1.min(other.1), self.1.max(other.1));
        for point in boundset.iter() {
            if point.in_bounds_strict((x1, y1), (x2, y2)) {
                return false;
            }
        }
        true
    }

    fn rect_in_full_boundset(&self, other: &Point, boundset: &HashSet<Point>) -> bool {
        // i think we get away with only checking the outer line
        let (x1, x2) = (self.0.min(other.0), self.0.max(other.0));
        let (y1, y2) = (self.1.min(other.1), self.1.max(other.1));
        //mt_log!(Level::Debug, "{:?}, {:?}", (x1, y1), (x2, y2));
        for x in x1..x2 + 1 {
            if !boundset.contains(&Point(x, y1)) {
                return false;
            }
        }

        for x in x1..x2 + 1 {
            if !boundset.contains(&Point(x, y2)) {
                return false;
            }
        }

        for y in y1..(y2 + 1) {
            if !boundset.contains(&Point(x1, y)) {
                return false;
            }
        }

        for y in y1..y2 + 1 {
            if !boundset.contains(&Point(x2, y)) {
                return false;
            }
        }
        true
    }
}
struct Field {
    positions: Vec<Point>,
}

impl Field {
    fn from_string(s: &str) -> Field {
        Field {
            positions: s
                .lines()
                .map(|line| {
                    let numbers: Vec<isize> = line
                        .split(',')
                        .map(|sn| sn.parse::<isize>().unwrap())
                        .collect();
                    Point(numbers[0], numbers[1])
                })
                .collect(),
        }
    }

    fn find_pairs(&self) -> Vec<(Point, Point)> {
        let mut pairs = Vec::new();
        for (i, &point) in self.positions.iter().enumerate() {
            for &point_b in self.positions.iter().skip(i) {
                pairs.push((point, point_b));
            }
        }
        pairs
    }

    fn part1(&self) {
        let mut pairs: Vec<usize> = self
            .find_pairs()
            .iter()
            .map(|pair| {
                let (point_a, point_b) = pair;
                point_a.size_rectangle(point_b)
            })
            .collect();
        pairs.sort();
        pairs.reverse();
        mt_log!(Level::Info, "Result Part 1: {:?}", pairs[0]);
    }

    fn part2(&self) {
        let mut green_set: HashSet<Point> = HashSet::new();
        let postion_set: HashSet<Point> = HashSet::from_iter(self.positions.iter().cloned());
        for i in 0..self.positions.len() - 1 {
            let current_point = self.positions[i];
            let next_point = self.positions[i + 1];
            for point in current_point.between(&next_point) {
                green_set.insert(point);
            }
        }
        let current_point = self.positions.first().unwrap();
        let next_point = self.positions.last().unwrap();
        for point in current_point.between(next_point) {
            green_set.insert(point);
        }
        mt_log!(Level::Info, "OutBounds Found");

        // get field bounds
        let (mut x1, mut x2, mut y1, mut y2) = (0, 0, 0, 0);
        for point in &self.positions {
            if point.0 < x1 {
                x1 = point.0;
            }
            if point.0 > x2 {
                x2 = point.0;
            }
            if point.1 < y1 {
                y1 = point.1;
            }
            if point.1 > y2 {
                y2 = point.1;
            }
        }
        let mut boundary = green_set.clone();
        boundary.extend(postion_set);
        mt_log!(Level::Info, "Full-field: {:?}, {:?}", (x1, y1), (x2, y2));
        mt_log!(Level::Info, "Calculating FullBound");

        // Find inside points
        boundary = fill_boundary(boundary, (x1, y1), (x2, y2));
        mt_log!(Level::Info, "FullBound calculated: {:?}", boundary);

        mt_log!(Level::Debug, "FullBoundary: {:?}", boundary);
        let mut pairs: Vec<usize> = self
            .find_pairs()
            .iter()
            .map(|pair| {
                let (point_a, point_b) = pair;
                if !point_a.rect_in_full_boundset(point_b, &boundary) {
                    //mt_log!(Level::Debug, "Rect was not in boundary");
                    return 0;
                }
                mt_log!(
                    Level::Debug,
                    "Possible Rect: {:?},{:?}, with Area: {}",
                    point_a,
                    point_b,
                    point_a.size_rectangle(point_b)
                );
                point_a.size_rectangle(point_b)
            })
            .collect();
        pairs.sort();
        pairs.reverse();
        mt_log!(Level::Info, "Result Part 2: {:?}", pairs[0]);
    }

    fn part2_v2(&self) {
        let mut green_set: HashSet<Point> = HashSet::new();
        let postion_set: HashSet<Point> = HashSet::from_iter(self.positions.iter().cloned());
        for i in 0..self.positions.len() - 1 {
            let current_point = self.positions[i];
            let next_point = self.positions[i + 1];
            for point in current_point.between(&next_point) {
                green_set.insert(point);
            }
        }
        let current_point = self.positions.first().unwrap();
        let next_point = self.positions.last().unwrap();
        for point in current_point.between(next_point) {
            green_set.insert(point);
        }
        mt_log!(Level::Info, "OutBounds Found");

        // get field bounds
        let (mut x1, mut x2, mut y1, mut y2) = (0, 0, 0, 0);
        for point in &self.positions {
            if point.0 < x1 {
                x1 = point.0;
            }
            if point.0 > x2 {
                x2 = point.0;
            }
            if point.1 < y1 {
                y1 = point.1;
            }
            if point.1 > y2 {
                y2 = point.1;
            }
        }
        let mut boundary = green_set.clone();
        boundary.extend(postion_set);
        mt_log!(Level::Info, "Full-field: {:?}, {:?}", (x1, y1), (x2, y2));

        mt_log!(Level::Debug, "Boundary: {:?}", boundary);
        let mut pairs: Vec<usize> = self
            .find_pairs()
            .iter()
            .map(|pair| {
                let (point_a, point_b) = pair;
                if !point_a.rect_in_boundset(point_b, &boundary) {
                    //mt_log!(Level::Debug, "Rect was not in boundary");
                    return 0;
                }
                mt_log!(
                    Level::Debug,
                    "Possible Rect: {:?},{:?}, with Area: {}",
                    point_a,
                    point_b,
                    point_a.size_rectangle(point_b)
                );
                point_a.size_rectangle(point_b)
            })
            .collect();
        pairs.sort();
        pairs.reverse();
        mt_log!(Level::Info, "Result Part 2: {:?}", pairs[0]);
    }
}

fn get_possible_neighbours(p: &Point) -> Vec<Point> {
    vec![
        p.add(&Point(1, 0)),
        p.add(&Point(-1, 0)),
        p.add(&Point(0, 1)),
        p.add(&Point(1, 0)),
        p.add(&Point(1, 1)),
        p.add(&Point(-1, 1)),
        p.add(&Point(1, -1)),
        p.add(&Point(-1, -1)),
    ]
}

fn fill_boundary(
    mut boundary: HashSet<Point>,
    top_left: (isize, isize),
    bottom_right: (isize, isize),
) -> HashSet<Point> {
    // find first point inside
    let mut explorable_set: Vec<Point> = Vec::new();
    for point in get_possible_neighbours(boundary.iter().last().unwrap()) {
        if point_is_in_boundary(&point, &boundary, top_left, bottom_right) {
            mt_log!(Level::Info, "Found first point: {:?}", point);
            explorable_set.push(point);
            break;
        }
    }
    while let Some(next_node) = explorable_set.pop() {
        for n_point in get_possible_neighbours(&next_node) {
            if !boundary.contains(&n_point) {
                boundary.insert(n_point);
                explorable_set.push(n_point);
            }
        }
    }
    boundary
}

fn find_hit_in_direction(
    p: &Point,
    direction: Point,
    boundary: &HashSet<Point>,
    top_left: (isize, isize),
    bottom_right: (isize, isize),
) -> bool {
    let mut point = *p;
    loop {
        //mt_log!(Level::Debug, "Checking Point: {:?}", point);
        if boundary.contains(&point) {
            return true;
        }
        point = point.add(&direction);
        if !point.in_bounds(top_left, bottom_right) {
            return false;
        }
    }
}

fn point_is_in_boundary(
    p: &Point,
    boundary: &HashSet<Point>,
    top_left: (isize, isize),
    bottom_right: (isize, isize),
) -> bool {
    if boundary.contains(p) {
        return false;
    }
    if !find_hit_in_direction(p, Point(0, 1), boundary, top_left, bottom_right) {
        return false;
    }
    if !find_hit_in_direction(p, Point(0, -1), boundary, top_left, bottom_right) {
        return false;
    }
    if !find_hit_in_direction(p, Point(1, 0), boundary, top_left, bottom_right) {
        return false;
    }
    if !find_hit_in_direction(p, Point(-1, 0), boundary, top_left, bottom_right) {
        return false;
    }
    true
}

fn main() {
    mt_new!(None, Level::Info, OutputStream::StdOut, true);
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        mt_log!(Level::Error, "Usage: {} <input.txt>", args[0]);
    }
    let filepath = &args[1];
    let filecontent = std::fs::read_to_string(filepath).expect("Could not read file");
    let field = Field::from_string(&filecontent);
    field.part1();
    field.part2_v2();
    mt_flush!().unwrap();
}
