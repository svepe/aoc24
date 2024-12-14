use std::collections::HashSet;
use std::fs;
use std::ops::{Add, Sub};
use gcd::binary_u32;


#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Position {
    row: i32,
    col: i32,
}

impl Add<&Position> for Position {
    type Output = Position;
    fn add(self, other: &Position) -> Position {
        Position {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Sub<&Position> for Position {
    type Output = Position;
    fn sub(self, other: &Position) -> Position {
        Position {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl Position {
    fn simplify(&mut self) {
        let gcd = binary_u32(self.row.unsigned_abs(), self.col.unsigned_abs());
        self.row /= gcd as i32;
        self.col /= gcd as i32;
    }
}

#[derive(Debug)]
struct Antena {
    kind: char,
    position: Position,
}

#[derive(Debug)]
struct Map {
    antennae: Vec<Antena>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn contains(&self, position: &Position) -> bool {
        (0..self.rows as i32).contains(&position.row)
            && (0..self.cols as i32).contains(&position.col)
    }
}

fn read_input(filename: &str) -> Map {
    let contents = fs::read_to_string(filename).expect("Unable to read input file");

    let mut antennae = vec![];

    let lines: Vec<&str> = contents.trim().lines().collect();
    let (rows, cols) = (lines.len(), lines.first().map_or(0, |line| line.len()));

    for (i, line) in lines.iter().enumerate() {
        for (j, kind) in line.chars().enumerate() {
            if kind == '.' {
                continue;
            }

            let position = Position {
                row: i as i32,
                col: j as i32,
            };
            antennae.push(Antena { kind, position });
        }
    }

    Map {
        antennae,
        rows,
        cols,
    }
}

fn get_antinodes_in_pair(a: &Antena, b: &Antena, map: &Map) -> Vec<Position> {
    let delta = b.position - &a.position;

    let mut antinodes = vec![];

    let antinode_a = a.position - &delta;
    if map.contains(&antinode_a) {
        antinodes.push(antinode_a);
    }

    let antinode_b = b.position + &delta;
    if map.contains(&antinode_b) {
        antinodes.push(antinode_b);
    }

    antinodes
}

fn get_antinodes_in_line(a: &Antena, b: &Antena, map: &Map) -> Vec<Position> {
    let mut delta = b.position - &a.position;
    delta.simplify();

    let mut antinodes = vec![];

    let mut antinode = a.position;
    while map.contains(&antinode) {
        antinodes.push(antinode);
        antinode = antinode - &delta;
    }

    antinode = a.position + &delta;
    while map.contains(&antinode) {
        antinodes.push(antinode);
        antinode = antinode + &delta;
    }

    antinodes
}

fn solve(map: &Map, generate_antinodes: fn(&Antena, &Antena, &Map) -> Vec<Position>) -> usize {
    let kinds = map
        .antennae
        .iter()
        .map(|antena| antena.kind)
        .collect::<HashSet<_>>();

    let mut antinodes = HashSet::new();

    for kind in kinds {
        let mut antennae = map
            .antennae
            .iter()
            .filter(|antena| antena.kind == kind)
            .collect::<Vec<_>>();

        antennae.sort_by(|a, b| {
            if a.position.row == b.position.row {
                a.position.col.cmp(&b.position.col)
            } else {
                a.position.row.cmp(&b.position.row)
            }
        });

        for i in 0..antennae.len() {
            for j in i + 1..antennae.len() {
                antinodes.extend(generate_antinodes(antennae[i], antennae[j], map));
            }
        }
    }

    antinodes.len()
}

fn main() {
    let map = read_input("input.txt");

    let answer = solve(&map, get_antinodes_in_pair);
    println!("Part 1 answer is: {answer}");

    let answer = solve(&map, get_antinodes_in_line);
    println!("Part 2 answer is: {answer}");
}
