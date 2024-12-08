use regex::Regex;
use std::fs;

fn solve1(input: &str) -> usize {
    let re = Regex::new(r"mul\((?<X>\d{1,3}),(?<Y>\d{1,3})\)").unwrap();

    let mut sum = 0;
    for cap in re.captures_iter(input) {
        let x = cap["X"].parse::<usize>().expect("Invalid number");
        let y = cap["Y"].parse::<usize>().expect("Invalid number");
        sum += x * y;
    }

    sum
}

fn solve2(mut input: &str) -> usize {
    let mut sum = 0;
    let mut enabled = true;

    while !input.is_empty() {
        match input {
            s if s.starts_with("do()") => {
                input = &input[4..];
                enabled = true;
            }

            s if s.starts_with("don't()") => {
                input = &input[7..];
                enabled = false;
            }

            s if s.starts_with("mul") => {
                let re = Regex::new(r"^mul\((?<X>\d{1,3}),(?<Y>\d{1,3})\)").unwrap();

                match re.captures(input) {
                    Some(caps) if enabled => {
                        let x = caps["X"].parse::<usize>().expect("Invalid number");
                        let y = caps["Y"].parse::<usize>().expect("Invalid number");
                        sum += x * y;

                        input = &input[caps[0].len()..];
                    },
                    _ => input = &input[3..],
                }
            }
            _ => {
                input = &input[1..];
            }
        }
    }

    sum
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Unable to read input file");

    let sum = solve1(&contents);
    println!("Sum {sum}");

    let sum = solve2(&contents);
    println!("Sum {sum}");

}
