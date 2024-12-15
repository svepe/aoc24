use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;

type Map = Vec<Vec<char>>;

fn read_input(filename: &str) -> Map {
    let contents = fs::read_to_string(filename).expect("Unable to read input file");
    contents
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec()
}
fn get_neighbours(row: usize, col: usize, map: &Map) -> Vec<(usize, usize)> {
    let mut res = vec![];

    if row > 0 {
        res.push((row - 1, col));
    }

    if row < map.len() - 1 {
        res.push((row + 1, col));
    }

    if col > 0 {
        res.push((row, col - 1));
    }

    if col < map[0].len() - 1 {
        res.push((row, col + 1));
    }

    res
}

fn floodfill_region(
    row: usize,
    col: usize,
    map: &Map,
    used: &mut [Vec<bool>],
) -> Vec<(usize, usize)> {
    let name = map[row][col];

    let mut region = vec![];

    let mut frontier = VecDeque::new();
    frontier.push_back((row, col));

    while let Some(cell) = frontier.pop_front() {
        let (row, col) = cell;

        if used[row][col] {
            continue;
        }

        region.push(cell);
        used[row][col] = true;

        for neighbour in get_neighbours(row, col, map) {
            let (row, col) = neighbour;
            if map[row][col] == name {
                frontier.push_back(neighbour);
            }
        }
    }

    region
}

fn get_perimeter(region: &[(usize, usize)], map: &Map) -> usize {
    let mut res = 0;
    for cell in region {
        let (row, col) = *cell;
        let name = map[row][col];
        let mut border = 4;
        for neighbour in get_neighbours(row, col, map) {
            let (row, col) = neighbour;
            if map[row][col] != name {
                res += 1;
            }
            border -= 1;
        }
        res += border;
    }
    res
}

fn count_corners_per_tile(tile: &[Vec<bool>]) -> usize {
    // For any tile centered at a cell X
    //
    // ? ? ?
    // ? X ?
    // ? ? ?
    //
    // there are for signatures, namely
    //
    // TL:    TR:
    // b c    a b
    // a X    X c
    //
    // BL:    BR:
    // c X    X a
    // b a    c b
    //
    // We should count a corner in the following cases
    //
    // a b | 0 0 | 0 X | X 0
    // X c | X 0 | X 0 | X X
    //
    // a | b | c
    // 0 | _ | 0 => corner
    // 1 | 0 | 1 => corner

    let border = [
        (0, 1),
        (0, 2),
        (1, 2),
        (2, 2),
        (2, 1),
        (2, 0),
        (1, 0),
        (0, 0),
        (0, 1),
    ];

    border
        .map(|(i, j)| tile[i][j])
        .iter()
        .tuple_windows()
        .step_by(2)
        .map(|(&a, &b, &c)| matches!((a, b, c), (false, _, false) | (true, false, true)) as usize)
        .sum()
}

fn count_sides(region: &[(usize, usize)], map: &Map) -> usize {
    let height = map.len();
    let width = map[0].len();
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    region
        .iter()
        .map(|(row, col)| {
            deltas
                .map(|(dr, dc)| {
                    let r = *row as isize + dr;
                    let c = *col as isize + dc;
                    if (0..height as isize).contains(&r) && (0..width as isize).contains(&c) {
                        region.contains(&(r as usize, c as usize))
                    } else {
                        false
                    }
                })
                .chunks(3)
                .map(|chunk| chunk.to_vec())
                .collect_vec()
        })
        .map(|tile| count_corners_per_tile(&tile))
        .sum()
}

fn get_area(region: &[(usize, usize)]) -> usize {
    region.len()
}

fn solve(map: &Map, score_perimeter: fn(&[(usize, usize)], &Map) -> usize) -> usize {
    let mut used = vec![vec![false; map[0].len()]; map.len()];
    let mut regions = vec![];

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if used[i][j] {
                continue;
            }
            regions.push(floodfill_region(i, j, map, &mut used));
        }
    }

    let mut res = 0;
    for region in &regions {
        res += score_perimeter(region, map) * get_area(region);
    }
    res
}

fn main() {
    let map = read_input("input.txt");

    let answer = solve(&map, get_perimeter);
    println!("Part 1 answer is: {answer}");

    let answer = solve(&map, count_sides);
    println!("Part 2 answer is: {answer}");
}
