use std::collections::HashSet;
use std::fs;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
enum Orientation {
    North,
    West,
    South,
    East,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Position {
    row: isize,
    col: isize,
}

impl Add<(isize, isize)> for Position {
    type Output = Position;
    fn add(self, other: (isize, isize)) -> Position {
        Position {
            row: self.row + other.0,
            col: self.col + other.1,
        }
    }
}

impl Sub<(isize, isize)> for Position {
    type Output = Position;
    fn sub(self, other: (isize, isize)) -> Position {
        Position {
            row: self.row - other.0,
            col: self.col - other.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    orientation: Orientation,
    position: Position,
}

impl State {
    fn get_step(&self) -> (isize, isize) {
        match self.orientation {
            Orientation::North => (-1, 0),
            Orientation::West => (0, -1),
            Orientation::South => (1, 0),
            Orientation::East => (0, 1),
        }
    }

    fn move_to(&mut self, obstacle: &Position) {
        self.position = obstacle.clone() - self.get_step();
    }

    fn turn_cw(&mut self) {
        self.orientation = match self.orientation {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        };
    }
}

#[derive(Debug, Clone)]
struct Map {
    obstacles: HashSet<Position>,
    width: isize,
    height: isize,
}

impl Map {
    fn contains(&self, position: &Position) -> bool {
        (0..self.height).contains(&position.row) && (0..self.width).contains(&position.col)
    }

    fn get_obstacle_ahead(&self, state: &State) -> Option<Position> {
        let ahead = |p: &&Position| match state.orientation {
            Orientation::North => p.col == state.position.col && p.row < state.position.row,
            Orientation::West => p.row == state.position.row && p.col < state.position.col,
            Orientation::South => p.col == state.position.col && p.row > state.position.row,
            Orientation::East => p.row == state.position.row && p.col > state.position.col,
        };

        let obstacles_ahead = self.obstacles.iter().filter(ahead);

        match state.orientation {
            Orientation::North => obstacles_ahead.max_by_key(|p| p.row),
            Orientation::West => obstacles_ahead.max_by_key(|p| p.col),
            Orientation::South => obstacles_ahead.min_by_key(|p| p.row),
            Orientation::East => obstacles_ahead.min_by_key(|p| p.col),
        }
        .cloned()
    }
}

fn parse_input(input: &str) -> (Map, State) {
    let mut state = None;
    let mut map = Map {
        obstacles: HashSet::new(),
        width: 0,
        height: 0,
    };

    for (i, line) in input.trim().split("\n").enumerate() {
        for (j, cell) in line.chars().enumerate() {
            let position = Position {
                row: i as isize,
                col: j as isize,
            };

            match cell {
                '#' => {
                    map.obstacles.insert(position);
                }
                '^' => {
                    state = Some(State {
                        orientation: Orientation::North,
                        position,
                    })
                }
                '>' => {
                    state = Some(State {
                        orientation: Orientation::East,
                        position,
                    })
                }
                'v' => {
                    state = Some(State {
                        orientation: Orientation::South,
                        position,
                    })
                }
                '<' => {
                    state = Some(State {
                        orientation: Orientation::West,
                        position,
                    })
                }
                _ => (),
            };

            if map.height == 0 {
                map.width += 1;
            }
        }
        map.height += 1;
    }

    (map, state.expect("Guard must have an initial state"))
}

fn can_loop(map: &Map, initial_state: &State) -> bool {
    let mut state = initial_state.clone();
    let mut corners: Vec<State> = vec![];

    while let Some(obstacle) = map.get_obstacle_ahead(&state) {
        state.move_to(&obstacle);

        if corners.contains(&state) {
            return true;
        }

        corners.push(state.clone());

        state.turn_cw();
    }

    false
}

fn solve1(map: &Map, initial_state: &State) -> usize {
    let mut state = initial_state.clone();
    let mut traversed = HashSet::<Position>::new();

    while map.contains(&state.position) {
        traversed.insert(state.position.clone());

        let next_position = state.position.clone() + state.get_step();

        if !map.obstacles.contains(&next_position) {
            state.position = next_position;
        } else {
            state.turn_cw();
        }
    }

    traversed.len()
}

fn solve2(map: &Map, initial_state: &State) -> usize {
    let mut state = initial_state.clone();
    let mut num_loops = 0;
    let mut traversed = HashSet::<Position>::new();

    while map.contains(&state.position) {
        traversed.insert(state.position.clone());

        let next_position = state.position.clone() + state.get_step();

        if !traversed.contains(&next_position) && map.contains(&next_position) {
            let mut map = map.clone();
            map.obstacles.insert(state.position.clone() + state.get_step());
            if can_loop(&map, &state) {
                num_loops += 1;
            }
        }

        if !map.obstacles.contains(&next_position) {
            state.position = next_position;
        } else {
            state.turn_cw();
        }
    }

    num_loops
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Unable to read input file");
    let (map, guard) = parse_input(&contents);

    let answer = solve1(&map, &guard);
    println!("Answer is {answer}");

    let answer = solve2(&map, &guard);
    println!("Answer is {answer}");
}
