use regex::Regex;
use std::fs;

#[derive(Debug, Copy, Clone)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn solve(&self) -> i64 {
        let det = self.button_a.0 * self.button_b.1 - self.button_b.0 * self.button_a.1;

        let a = self.button_b.1 * self.prize.0 - self.button_b.0 * self.prize.1;
        let b = -self.button_a.1 * self.prize.0 + self.button_a.0 * self.prize.1;

        if a % det != 0 || b % det != 0 {
            return 0;
        }

        a / det * 3 + b / det
    }
}

fn parse_machine<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Option<Machine> {
    let [line_a, line_b, line_p] = [lines.next()?, lines.next()?, lines.next()?];

    let a_regex = Regex::new(r"Button A: X\+(?P<x_a>\d+), Y\+(?P<y_a>\d+)").unwrap();
    let b_regex = Regex::new(r"Button B: X\+(?P<x_b>\d+), Y\+(?P<y_b>\d+)").unwrap();
    let p_regex = Regex::new(r"Prize: X=(?P<x_p>\d+), Y=(?P<y_p>\d+)").unwrap();

    let captures = a_regex.captures(line_a)?;
    let x_a = captures.name("x_a")?.as_str().parse().ok()?;
    let y_a = captures.name("y_a")?.as_str().parse().ok()?;

    let captures = b_regex.captures(line_b)?;
    let x_b = captures.name("x_b")?.as_str().parse().ok()?;
    let y_b = captures.name("y_b")?.as_str().parse().ok()?;

    let captures = p_regex.captures(line_p)?;
    let x_p = captures.name("x_p")?.as_str().parse().ok()?;
    let y_p = captures.name("y_p")?.as_str().parse().ok()?;

    let _ = lines.next();

    Some(Machine {
        button_a: (x_a, y_a),
        button_b: (x_b, y_b),
        prize: (x_p, y_p),
    })
}

fn read_input(filename: &str) -> Vec<Machine> {
    let contents = fs::read_to_string(filename).expect("Unable to read input file");
    let mut lines = contents.lines();
    let mut machines = vec![];

    // Process all coordinate groups
    while let Some(machine) = parse_machine(&mut lines) {
        println!("Button A: {:?}", machine.button_a);
        println!("Button B: {:?}", machine.button_b);
        println!("Prize: {:?}", machine.prize);
        machines.push(machine);
    }

    machines
}

fn solve1(machines: &[Machine]) -> i64 {
    machines.iter().map(|machine| machine.solve()).sum()
}

fn solve2(machines: &[Machine]) -> i64 {
    let mut modified_machines = machines.to_vec();
    modified_machines.iter_mut().for_each(|machine| {
        machine.prize.0 += 10_000_000_000_000;
        machine.prize.1 += 10_000_000_000_000;
    });
    modified_machines.iter().map(|machine| machine.solve()).sum()
}

fn main() {
    let machines = read_input("input.txt");

    let answer = solve1(&machines);
    println!("Part 1 answer is: {answer}");

    let answer = solve2(&machines);
    println!("Part 2 answer is: {answer}");
}
