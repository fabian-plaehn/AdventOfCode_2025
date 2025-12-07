use mt_logger::*;

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
}

#[derive(Debug)]
struct TachyonManifold {
    objects: Vec<Objects>,
    start: usize,
    splitters: Vec<usize>,
    beams: Vec<usize>,
    max_y: usize,
    open_beams: Vec<usize>,
}

impl TachyonManifold {
    fn from_string(s: &str) -> TachyonManifold {
        let mut objects: Vec<Objects> = Vec::new();
        let mut start: usize = 0;
        let mut splitters: Vec<usize> = Vec::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    objects.push(Objects::Start(Start {
                        position: (x, y),
                        child: None,
                    }));
                    start = objects.len() - 1;
                } else if c == '^' {
                    objects.push(Objects::Splitter(Splitter {
                        position: (x, y),
                        hit: false,
                        child_left: None,
                        child_right: None,
                    }));
                    splitters.push(objects.len() - 1);
                }
            }
        }
        TachyonManifold {
            objects,
            start,
            splitters,
            beams: Vec::new(),
            max_y: s.lines().count(),
            open_beams: Vec::new(),
        }
    }

    fn get_spitters(&self) -> Vec<Splitter> {
        self.splitters
            .iter()
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
        let mut count = 0;
        let mut created_beams = 0;
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
            if let Some(splitter) = self
                .get_spitters()
                .iter()
                .find(|s| s.position == new_position)
            {
                mt_log!(Level::Debug, "Splitter got hit at {:?}", new_position);
                count += 1;

                self.create_beam((splitter.position.0 - 1, splitter.position.1), beam_index);
                self.create_beam((splitter.position.0 + 1, splitter.position.1), beam_index);
            } else {
                self.create_beam(new_position, beam_index);
            }
        }
        mt_log!(Level::Info, "Result Part 1: {}", count);
    }

    fn create_beam(&mut self, new_position: (usize, usize), parent_index: usize) -> usize {
        // check if beam already exists
        if self
            .get_beams()
            .iter()
            .any(|beam| beam.position == new_position)
        {
            mt_log!(Level::Debug, "Beam @ {:?} already exsits", new_position);
            return 0;
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
        1
    }
}

fn main() {
    mt_new!(None, Level::Debug, OutputStream::StdOut, true);
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
    mt_flush!().unwrap();
}
