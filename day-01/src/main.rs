use counter::Counter;
use std::{fmt, fs, io, iter::zip, num};

enum SolveError {
    Io(io::Error),
    LineParse,
    NumParse(num::ParseIntError),
}

impl From<io::Error> for SolveError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<num::ParseIntError> for SolveError {
    fn from(error: num::ParseIntError) -> Self {
        Self::NumParse(error)
    }
}

impl fmt::Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolveError::Io(err) => write!(f, "IO error: {err}"),
            SolveError::LineParse => write!(f, "Line parse error"),
            SolveError::NumParse(err) => write!(f, "Num parse error: {err}"),
        }
    }
}

fn parse_input(filename: &str) -> Result<(Vec<usize>, Vec<usize>), SolveError> {
    let contents = fs::read_to_string(filename)?;

    let mut v1 = vec![];
    let mut v2 = vec![];

    fn parse_helper(chunk: Option<&str>) -> Result<usize, SolveError> {
        let chunk = chunk.ok_or(SolveError::LineParse)?;
        Ok(chunk.parse::<usize>()?)
    }

    for line in contents.lines() {
        let mut split = line.split("   ");
        v1.push(parse_helper(split.next())?);
        v2.push(parse_helper(split.next())?);
    }

    Ok((v1, v2))
}

fn solve1() -> Result<usize, SolveError> {
    let (mut v1, mut v2) = parse_input("input.txt")?;
    v1.sort();
    v2.sort();

    let manhattan_dist = |acc, (a, b)| acc + (a as isize - b as isize).unsigned_abs();
    let answer = zip(v1, v2).fold(0, manhattan_dist);

    Ok(answer)
}

fn solve2() -> Result<usize, SolveError> {
    let (v1, v2) = parse_input("input.txt")?;
    let v1_counts = v1.iter().collect::<Counter<_>>();
    let v2_counts = v2.iter().collect::<Counter<_>>();

    let mut answer = 0;
    for (num1, count1) in v1_counts.iter() {
        answer += *num1 * count1 * v2_counts[num1];
    }

    Ok(answer)
}

fn main() {
    match solve2() {
        Ok(answer) => println!("{answer}"),
        Err(err) => println!("{err}"),
    };
}
