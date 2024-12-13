use std::collections::VecDeque;
use std::fmt;
use std::fs;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concatenate => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Multiply => write!(f, "*"),
            Operation::Concatenate => write!(f, "|"),
        }
    }
}

#[derive(Debug, Default)]
struct Expression {
    operands: Vec<u64>,
    operators: Vec<Operation>,
}

impl Expression {
    fn from(operands: &[u64], target: u64, possible_operators: &[Operation]) -> Option<Expression> {
        let expression = Expression {
            operands: vec![operands[0]],
            operators: vec![],
        };

        let mut candidates = VecDeque::from([expression]);

        while let Some(candidate) = candidates.pop_front() {
            let value = candidate.calculate();

            if value == target {
                return Some(candidate);
            }

            // Cannot get smaller value with + and *
            if value > target {
                continue;
            }

            // No more operands left
            if candidate.operands.len() == operands.len() {
                continue;
            }

            for op in possible_operators {
                let mut candidate_operands = candidate.operands.clone();
                candidate_operands.push(operands[candidate_operands.len()]);

                let mut candidate_operators = candidate.operators.clone();
                candidate_operators.push(op.clone());

                let proposed_candidate = Expression {
                    operands: candidate_operands,
                    operators: candidate_operators,
                };
                candidates.push_back(proposed_candidate);
            }
        }

        None
    }

    fn calculate(&self) -> u64 {
        let op_arg_pairs = self.operators.iter().zip(self.operands[1..].iter());
        op_arg_pairs.fold(self.operands[0], |res, (op, arg)| op.apply(res, *arg))
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op_arg_pairs = self.operators.iter().zip(self.operands[1..].iter());
        let text = op_arg_pairs.fold(self.operands[0].to_string(), |acc, (op, arg)| {
            acc + &format!(" {} {}", op, arg)
        });
        write!(f, "{text}")
    }
}

fn read_input(filename: &str) -> (Vec<Vec<u64>>, Vec<u64>) {
    let contents = fs::read_to_string(filename).expect("Unable to read input file");
    let mut operands = vec![];
    let mut targets = vec![];
    for line in contents.trim().split("\n") {
        if let Some((target, expression_operands)) = line.split_once(":") {
            let target: u64 = target.trim().parse().expect("invalid target value");
            targets.push(target);

            let expression_operands: Vec<u64> = expression_operands
                .trim()
                .split(" ")
                .map(|v| v.parse().expect("invalid operand"))
                .collect();
            operands.push(expression_operands);
        }
    }
    (operands, targets)
}

fn solve(operands: &[Vec<u64>], targets: &[u64], possible_operators: &[Operation]) -> u64 {
    let mut sum = 0;
    for (expression_operands, target) in operands.iter().zip(targets.iter()) {
        if let Some(expression) = Expression::from(expression_operands, *target, possible_operators)
        {
            println!("{expression} = {target}");
            sum += target;
        }
    }
    sum
}

fn main() {
    let (operands, targets) = read_input("input.txt");

    let mut allowed_operators = vec![Operation::Add, Operation::Multiply];
    let answer = solve(&operands, &targets, &allowed_operators);
    println!("Part 1 answer is: {answer}");

    allowed_operators.push(Operation::Concatenate);
    let answer = solve(&operands, &targets, &allowed_operators);
    println!("Part 2 answer is: {answer}");
}
