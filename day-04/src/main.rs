use itertools::Itertools;
use std::collections::LinkedList;
use std::fs;

fn solve1(grid: &[Vec<char>]) -> usize {
    #[derive(Debug)]
    struct Solution {
        cell: (usize, usize),
        step: (isize, isize),
        word: String,
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut candidates: LinkedList<Solution> = LinkedList::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, letter) in row.iter().enumerate() {
            if *letter != 'X' {
                continue;
            }

            let steps = (-1isize..=1).cartesian_product(-1isize..=1);
            for step in steps {
                if step.0 == 0 && step.1 == 0 {
                    continue;
                }

                candidates.push_back(Solution {
                    cell: (i, j),
                    word: String::from(*letter),
                    step,
                })
            }
        }
    }

    let mut solutions: LinkedList<Solution> = LinkedList::new();

    while let Some(candidate) = candidates.pop_front() {
        match &candidate.word {
            word if word == "XMAS" => {
                solutions.push_back(candidate);
            }

            word if word.len() < 4 => {
                let i = candidate.cell.0 as isize + candidate.step.0;
                let j = candidate.cell.1 as isize + candidate.step.1;

                if i < 0 || (height as isize) <= i {
                    continue;
                }

                if j < 0 || (width as isize) <= j {
                    continue;
                }

                let i = i as usize;
                let j = j as usize;

                let new_candidate = Solution {
                    cell: (i, j),
                    word: candidate.word.clone() + &grid[i][j].to_string(),
                    step: candidate.step,
                };

                candidates.push_back(new_candidate);
            }

            _ => (),
        }
    }

    solutions.len()
}

fn solve2(grid: &[Vec<char>]) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut num_xmas = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if grid[i][j] != 'A' {
                continue;
            }

            // a . b
            // . A .
            // c . d

            let a = grid[i - 1][j - 1];
            let b = grid[i - 1][j + 1];
            let c = grid[i + 1][j - 1];
            let d = grid[i + 1][j + 1];

            if !matches!([a, d], ['S', 'M'] | ['M', 'S']) {
                continue;
            }

            if !matches!([b, c], ['S', 'M'] | ['M', 'S']) {
                continue;
            }

            num_xmas += 1;
        }
    }
    num_xmas
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Unable to read input file");

    let mut grid: Vec<Vec<char>> = vec![];

    for line in contents.trim().split("\n") {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let answer = solve1(&grid);
    println!("Answer is {}", answer);

    let answer = solve2(&grid);
    println!("Answer is {}", answer);
}
