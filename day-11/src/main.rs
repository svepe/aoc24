use std::fs;
use std::collections::HashMap;

fn read_input(filename: &str) -> Vec<usize> {
    let contents = fs::read_to_string(filename).expect("Unable to read input file");
    contents
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn transition_stone(num: &usize) -> Vec<usize> {
    if *num == 0 {
        return vec![1];
    }

    let str = num.to_string();
    if str.len() % 2 != 0 {
        return vec![num * 2024];
    }

    let (left, right) = str.split_at(str.len() / 2);
    vec![left.parse().unwrap(), right.parse().unwrap()]
}

fn solve(sequence: &[usize], remaining_iters: usize, atlas: &mut HashMap<(usize, usize), usize>) -> usize {
    if remaining_iters == 0 {
        return sequence.len();
    }

    let mut res = 0;
    for stone in sequence {
        if let Some(count) = atlas.get(&(*stone, remaining_iters)) {
            res += count;
        } else {
            let count = solve(&transition_stone(stone), remaining_iters - 1, atlas);
            atlas.insert((*stone, remaining_iters), count);
            res += count;
        }
    }
    res
}

fn main() {
    let sequence = read_input("input.txt");

    let mut atlas = HashMap::new();
    let answer = solve(&sequence, 25, &mut atlas);
    println!("Part 1 answer is: {answer}");

    let mut atlas = HashMap::new();
    let answer = solve(&sequence, 75, &mut atlas);
    println!("Part 2 answer is: {answer}");
}
