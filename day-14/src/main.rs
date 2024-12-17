use regex::Regex;
use std::fs;
use std::ops::{Add, Mul, Rem};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        Self {
            x: scalar * self.x,
            y: scalar * self.y,
        }
    }
}

impl Rem for Vec2 {
    type Output = Self;

    fn rem(self, modulo: Self) -> Self {
        Self {
            x: self.x % modulo.x,
            y: self.y % modulo.y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    position: Vec2,
    velocity: Vec2,
}

impl Robot {
    fn r#move(&self, duration: u32, width: u32, height: u32) -> Self {
        let t = duration as i32;
        let size = Vec2 {
            x: width as i32,
            y: height as i32,
        };
        Robot {
            position: (self.position + (self.velocity + size) * t) % size,
            velocity: self.velocity,
        }
    }
}

fn parse_robot<'a>(line: &str) -> Option<Robot> {
    let regex =
        Regex::new(r"p=(?P<p_x>-?\d+),(?P<p_y>-?\d+) v=(?P<v_x>-?\d+),(?P<v_y>-?\d+)").unwrap();
    let captures = regex.captures(line)?;
    let p_x = captures.name("p_x")?.as_str().parse().ok()?;
    let p_y = captures.name("p_y")?.as_str().parse().ok()?;
    let v_x = captures.name("v_x")?.as_str().parse().ok()?;
    let v_y = captures.name("v_y")?.as_str().parse().ok()?;

    Some(Robot {
        position: Vec2 { x: p_x, y: p_y },
        velocity: Vec2 { x: v_x, y: v_y },
    })
}

fn read_input(filename: &str) -> Vec<Robot> {
    let contents = fs::read_to_string(filename).expect("Unable to read input file");
    contents.lines().filter_map(parse_robot).collect()
}

fn solve1(robots: &[Robot], width: u32, height: u32) -> i64 {
    let w = width as i32 / 2;
    let h = height as i32 / 2;
    let (num_q1, num_q2, num_q3, num_q4) = robots
        .iter()
        .map(|robot| robot.r#move(100, width, height))
        .fold((0, 0, 0, 0), |acc, robot| match robot.position {
            Vec2 { x, y } if x < w && y < h => (acc.0, acc.1 + 1, acc.2, acc.3),
            Vec2 { x, y } if x > w && y < h => (acc.0 + 1, acc.1, acc.2, acc.3),
            Vec2 { x, y } if x < w && y > h => (acc.0, acc.1, acc.2 + 1, acc.3),
            Vec2 { x, y } if x > w && y > h => (acc.0, acc.1, acc.2, acc.3 + 1),
            _ => acc,
        });

    num_q1 * num_q2 * num_q3 * num_q4
}

fn solve2(robots: &[Robot], width: u32, height: u32) -> i64 {
    let mut robots = robots.to_vec().clone();
    let w = width as i32 / 2;
    let h = height as i32 / 2;
    let mut iters = 1;
    let mut min_code = i32::MAX;

    loop {
        robots = robots
            .iter()
            .map(|robot| robot.r#move(1, width, height))
            .collect();
        let (num_q1, num_q2, num_q3, num_q4) =
            robots
                .iter()
                .fold((0, 0, 0, 0), |acc, robot| match robot.position {
                    Vec2 { x, y } if x < w && y < h => (acc.0, acc.1 + 1, acc.2, acc.3),
                    Vec2 { x, y } if x > w && y < h => (acc.0 + 1, acc.1, acc.2, acc.3),
                    Vec2 { x, y } if x < w && y > h => (acc.0, acc.1, acc.2 + 1, acc.3),
                    Vec2 { x, y } if x > w && y > h => (acc.0, acc.1, acc.2, acc.3 + 1),
                    _ => acc,
                });
        let code = num_q1 * num_q2 * num_q3 * num_q4;

        if code < min_code {
            min_code = code;

            println!("=======================");
            let mut map = vec![vec!['.'; width as usize]; height as usize];
            robots
                .iter()
                .for_each(|robot| map[robot.position.y as usize][robot.position.x as usize] = '*');

            for line in map {
                println!("{}", line.iter().collect::<String>());
            }
            println!("{iters}");

        }
        iters += 2;

        if iters > 50000 {
            break 0;
        }
    }
}

fn main() {
    // let (robots, width, height) = (read_input("test.txt"), 11, 7);
    let (robots, width, height) = (read_input("input.txt"), 101, 103);

    let answer = solve1(&robots, width, height);
    println!("Part 1 answer is: {answer}");

    let answer = solve2(&robots, width, height);
    println!("Part 2 answer is: {answer}");
}
