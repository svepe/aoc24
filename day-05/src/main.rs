use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

type Rule = (usize, usize);
type Sequence = Vec<usize>;
type Requirements = HashMap<usize, HashSet<usize>>;

fn parse_rule(rule: &str) -> Rule {
    rule.split("|")
        .map(|page| page.parse::<usize>().expect("Unable to parse page number"))
        .collect_tuple::<Rule>()
        .expect("Rules must be valid")
}

fn parse_seq(seq: &str) -> Sequence {
    seq.split(",")
        .map(|page| page.parse::<usize>().expect("Unable to parse page number"))
        .collect_vec()
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Sequence>) {
    let mut rules: Vec<Rule> = vec![];
    let mut seqs: Vec<Sequence> = vec![];

    let mut reading_rules = true;
    for line in input.trim().split("\n") {
        match line {
            "" => reading_rules = false,
            rule if reading_rules => {
                rules.push(parse_rule(rule));
            }
            seq if !reading_rules => {
                seqs.push(parse_seq(seq));
            }
            _ => (),
        }
    }

    (rules, seqs)
}

fn build_requirements(rules: &[Rule]) -> Requirements {
    let mut requirements = Requirements::new();

    for (prereq, page) in rules {
        match requirements.get_mut(page) {
            Some(page_prereqs) => {
                page_prereqs.insert(*prereq);
            }
            None => {
                requirements.insert(*page, HashSet::from([*prereq]));
            }
        };
    }

    requirements
}

fn is_valid(seq: &Sequence, requirements: &Requirements) -> bool {
    let mut available = HashSet::new();

    for page in seq {
        if let Some(prereqs) = requirements.get(page) {
            let seq_prereqs = HashSet::from_iter(prereqs.iter().filter(|p| seq.contains(p)));

            if !seq_prereqs.is_subset(&available) {
                return false;
            }
        }
        available.insert(page);
    }

    true
}

fn order(seq: &Sequence, requirements: &Requirements) -> Sequence {
    let mut res = seq.clone();

    while !is_valid(&res, requirements) {
        let mut available = HashSet::new();

        let mut i = 0usize;
        while i < res.len() {
            let page = res[i];

            let mut swapped = false;

            if let Some(prereqs) = requirements.get(&page) {
                let seq_prereqs =
                    HashSet::from_iter(prereqs.iter().filter(|p| seq.contains(p)).copied());
                let mut unsatisfied_reqs = &seq_prereqs - &available;

                let mut j = i + 1;
                while !unsatisfied_reqs.is_empty() && j < res.len() {
                    unsatisfied_reqs.remove(&res[j]);

                    if unsatisfied_reqs.is_empty() {
                        res.remove(i);
                        res.insert(j, page);
                        swapped = true;
                    }

                    j += 1;
                }
            }

            available.insert(page);
            if !swapped {
                i += 1;
            }
        }
    }

    res
}

fn solve1(seqs: &[Sequence], requirements: &Requirements) -> usize {
    seqs.iter()
        .filter(|seq| is_valid(seq, requirements))
        .fold(0usize, |acc, seq| acc + seq[seq.len() / 2])
}

fn solve2(seqs: &[Sequence], requirements: &Requirements) -> usize {
    seqs.iter()
        .filter(|seq| !is_valid(seq, requirements))
        .map(|seq| order(seq, requirements))
        .fold(0usize, |acc, seq| acc + seq[seq.len() / 2])
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Unable to read input file");
    let (rules, seqs) = parse_input(&contents);
    let requirements = build_requirements(&rules);

    let ans = solve1(&seqs, &requirements);
    println!("Answer is {ans}");

    let ans = solve2(&seqs, &requirements);
    println!("Answer is {ans}");
}
