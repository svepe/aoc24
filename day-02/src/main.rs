use itertools::Itertools;
use std::fs;

fn is_safe(report: &[isize]) -> bool {
    let mut diffs = report.iter().tuple_windows().peekable();
    let Some((first, second)) = diffs.peek() else {
        return true;
    };
    let is_increasing = second > first;
    diffs.all(|(first, second)| {
        let diff = second.abs_diff(*first);
        (1..=3).contains(&diff) && (second > first) == is_increasing
    })
}

fn solve(dampen: bool) -> usize {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Unable to read input file");

    let mut num_safe = 0usize;
    for line in contents.trim().split("\n") {
        let report = line
            .split_whitespace()
            .map(|x| x.parse::<isize>().expect("Invalid number"))
            .collect::<Vec<isize>>();

        if is_safe(&report) {
            num_safe += 1;
            continue;
        }

        if !dampen {
            continue;
        }

        for skip in 0..report.len() {
            let updated_report = report
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != skip)
                .map(|(_, report)| *report)
                .collect::<Vec<isize>>();

            if is_safe(&updated_report) {
                num_safe += 1;
                break;
            }
        }
    }
    num_safe
}

fn main() {
    let num_safe = solve(false);
    println!("{num_safe}");

    let num_safe = solve(true);
    println!("{num_safe}");
}
