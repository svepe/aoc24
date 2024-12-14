use itertools::Itertools;
use std::fs;
use std::collections::HashSet;

type Map = Vec<Vec<u32>>;

fn read_input(filename: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(filename).expect("Unable to read input file");
    contents
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect_vec()
        })
        .collect_vec()
}

fn score_trailhead(row: usize, col: usize, map: &Map) -> HashSet<(usize, usize)> {
    let cell = map[row][col];
    if cell == 9 {
        return HashSet::from([(row, col)]);
    }

    let mut res = HashSet::new();
    if row > 0 && cell + 1 == map[row-1][col] {
        res.extend(score_trailhead(row - 1, col, map));
    }

    if row < map.len() - 1 && cell + 1 == map[row + 1][col] {
        res.extend(score_trailhead(row + 1, col, map));
    }

    if col > 0 && cell + 1 == map[row][col - 1] {
        res.extend(score_trailhead(row, col - 1, map));
    }

    if col < map[0].len() - 1 && cell + 1 == map[row][col + 1] {
        res.extend(score_trailhead(row, col + 1, map));
    }

    res
}

fn rate_trailhead(row: usize, col: usize, map: &Map) -> usize {
    let cell = map[row][col];
    if cell == 9 {
        return 1;
    }

    let mut res = 0;
    if row > 0 && cell + 1 == map[row-1][col] {
        res += rate_trailhead(row - 1, col, map);
    }

    if row < map.len() - 1 && cell + 1 == map[row + 1][col] {
        res += rate_trailhead(row + 1, col, map);
    }

    if col > 0 && cell + 1 == map[row][col - 1] {
        res += rate_trailhead(row, col - 1, map);
    }

    if col < map[0].len() - 1 && cell + 1 == map[row][col + 1] {
        res += rate_trailhead(row, col + 1, map);
    }

    res
}

fn solve1(map: &Map) -> usize {
    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                res += score_trailhead(i, j, map).len();
            }
        }
    }
    res
}

fn solve2(map: &Map) -> usize {
    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                res += rate_trailhead(i, j, map);
            }
        }
    }
    res
}

fn main() {
    let map = read_input("test.txt");
    // println!("{map:?}");

    let answer = solve1(&map);
    println!("Part 1 answer is: {answer}");

    let answer = solve2(&map);
    println!("Part 2 answer is: {answer}");
}
