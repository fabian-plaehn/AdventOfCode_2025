use mt_logger::*;
use std::{collections::HashMap, mem};

#[derive(Debug, Clone, Copy)]
struct Start {
    position: (usize, usize),
    child: Option<usize>,
}
#[derive(Debug, Clone, Copy)]
struct Splitter {
    position: (usize, usize),
    hit: bool,
    child_left: Option<usize>,
    child_right: Option<usize>,
}
#[derive(Debug, Clone, Copy)]
struct Beam {
    position: (usize, usize),
    child: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Objects {
    Start(Start),
    Splitter(Splitter),
    Beam(Beam),
    None,
}

#[derive(Debug)]
struct TachyonManifold {
    objects: Vec<Objects>,
    start: usize,
    splitters: Vec<usize>,
    beams: Vec<usize>,
    max_y: usize,
    max_x: usize,
    open_beams: Vec<usize>,
    field: Vec<Vec<Objects>>,
}

impl TachyonManifold {
    fn from_string(s: &str) -> TachyonManifold {
        let mut objects: Vec<Objects> = Vec::new();
        let mut start: usize = 0;
        let mut splitters: Vec<usize> = Vec::new();

        let max_y = s.lines().count();
        let max_x = s.lines().next().unwrap().chars().count();

        let mut field: Vec<Vec<Objects>> = vec![vec![Objects::None; max_y]; max_x];

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    objects.push(Objects::Start(Start {
                        position: (x, y),
                        child: None,
                    }));
                    start = objects.len() - 1;
                    field[x][y] = Objects::Start(Start {
                        position: (x, y),
                        child: None,
                    })
                } else if c == '^' {
                    objects.push(Objects::Splitter(Splitter {
                        position: (x, y),
                        hit: false,
                        child_left: None,
                        child_right: None,
                    }));
                    splitters.push(objects.len() - 1);
                    field[x][y] = Objects::Splitter(Splitter {
                        position: (x, y),
                        hit: false,
                        child_left: None,
                        child_right: None,
                    })
                }
            }
        }
        TachyonManifold {
            objects,
            start,
            splitters,
            beams: Vec::new(),
            max_y,
            max_x,
            open_beams: Vec::new(),
            field,
        }
    }

    fn get_spitters(&mut self) -> Vec<Splitter> {
        self.splitters
            .iter_mut()
            .map(|splitter_index| {
                let Objects::Splitter(splitter) = self.objects[*splitter_index] else {
                    panic!();
                };
                splitter
            })
            .collect::<Vec<Splitter>>()
    }

    fn get_beams(&self) -> Vec<&Beam> {
        self.beams
            .iter()
            .map(|beam_index| {
                let Objects::Beam(beam) = &self.objects[*beam_index] else {
                    panic!();
                };
                beam
            })
            .collect::<Vec<&Beam>>()
    }

    fn simulate(&mut self) {
        let Objects::Start(start) = &self.objects[self.start] else {
            mt_log!(
                Level::Error,
                "Start_Index: {}, Objects: {:?}",
                self.start,
                self.objects
            );
            mt_flush!().unwrap();
            panic!();
        };
        let beam: Beam = Beam {
            position: (start.position.0, start.position.1 + 1),
            child: None,
        };
        self.objects.push(Objects::Beam(beam));
        self.beams.push(self.objects.len() - 1);
        self.open_beams.push(self.objects.len() - 1);
        while let Some(beam_index) = self.open_beams.pop() {
            let new_position = {
                let Objects::Beam(beam) = self.objects.get_mut(beam_index).unwrap() else {
                    panic!();
                };

                (beam.position.0, beam.position.1 + 1)
            };
            if new_position.1 == self.max_y {
                mt_log!(Level::Debug, "Beam hit bottom");
                continue;
            }
            let splitter_index = self
                .objects
                .iter()
                .position(|o| matches!(o, Objects::Splitter(s) if s.position == new_position));

            if let Some(idx) = splitter_index {
                let (x, y) = {
                    let splitter = match &mut self.objects[idx] {
                        Objects::Splitter(s) => s,
                        _ => unreachable!(),
                    };

                    mt_log!(Level::Debug, "Splitter got hit at {:?}", new_position);
                    splitter.hit = true;
                    (splitter.position.0, splitter.position.1)
                };
                self.create_beam((x - 1, y), beam_index);
                self.create_beam((x + 1, y), beam_index);
            } else {
                self.create_beam(new_position, beam_index);
            }
        }

        let count = self.get_spitters().iter().filter(|s| s.hit).count();
        mt_log!(Level::Info, "Result Part 1: {}", count);
    }

    fn count_routes(
        &mut self,
        position: (usize, usize),
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        let (x, y) = position;
        if let Some(v) = memo.get(&position) {
            return *v;
        }
        mt_log!(Level::Debug, "{}", memo.capacity());
        if y == self.max_y {
            return 1;
        }

        match self.field[x][y] {
            Objects::Splitter(_) => {
                let (mut left, mut right) = (0, 0);
                if x > 0 {
                    left = self.count_routes((x - 1, y), memo);
                }
                if x < self.max_x {
                    right = self.count_routes((x + 1, y), memo);
                }
                memo.insert(position, left + right);
                left + right
            }
            _ => {
                let value = self.count_routes((x, y + 1), memo);
                memo.insert(position, value);
                value
            }
        }
    }
    fn create_beam(&mut self, new_position: (usize, usize), parent_index: usize) {
        if self
            .get_beams()
            .iter()
            .any(|beam| beam.position == new_position)
        {
            mt_log!(Level::Debug, "Beam @ {:?} already exsits", new_position);
            return;
        }
        let new_beam = Objects::Beam(Beam {
            position: new_position,
            child: None,
        });
        mt_log!(Level::Debug, "Created new Beam at {:?}", new_position);
        self.objects.push(new_beam);

        let new_child_index = self.objects.len() - 1;
        self.beams.push(new_child_index);
        self.open_beams.push(new_child_index);
        let Objects::Beam(beam) = self.objects.get_mut(parent_index).unwrap() else {
            panic!();
        };
        beam.child = Some(new_child_index);
    }
}

fn main() {
    mt_new!(None, Level::Info, OutputStream::StdOut, true);
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        mt_log!(Level::Error, "Usage {} <inputfile.txt>", args[0]);
        mt_flush!().unwrap();
        std::process::exit(1);
    }
    let filepath = &args[1];
    let filecontent = std::fs::read_to_string(filepath).expect("Could not read file");
    let mut tachyon_manifold = TachyonManifold::from_string(&filecontent);
    mt_log!(Level::Debug, "{:?}", tachyon_manifold);
    tachyon_manifold.simulate();

    // Own Idea was to simulate - allow "double beams" and count beams with y=max_y at the end but that took way too long
    // Unfortunatly i got this idea when i looked on reddit for ideas - i was tired
    let start_position = match tachyon_manifold
        .objects
        .get(tachyon_manifold.start)
        .unwrap()
    {
        Objects::Start(s) => s.position,
        _ => panic!(),
    };
    mt_log!(
        Level::Info,
        "Result Part 2: {}",
        tachyon_manifold.count_routes(start_position, &mut HashMap::new())
    );
    mt_flush!().unwrap();
}
