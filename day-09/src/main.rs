use itertools::Itertools;
use std::fmt;
use std::fs;

#[derive(Clone, PartialEq)]
enum Block {
    File(usize),
    Empty,
}

#[derive(Debug)]
enum Sector {
    File(usize, usize, usize, bool),
    Empty(usize, usize),
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Block::File(id) => write!(f, "{}", id),
            Block::Empty => write!(f, "-"),
        }
    }
}

fn read_input(filename: &str) -> Vec<Block> {
    let contents = fs::read_to_string(filename).expect("Unable to read input file");

    let mut blocks: Vec<Block> = vec![];

    let mut file_id = 0;
    for (i, digit) in contents.chars().enumerate() {
        if let Some(len) = digit.to_digit(10) {
            if i % 2 == 0 {
                blocks.append(&mut vec![Block::File(file_id); len as usize]);
                file_id += 1;
            } else {
                blocks.append(&mut vec![Block::Empty; len as usize]);
            }
        }
    }

    blocks
}

fn defragment_block(blocks: &mut [Block]) {
    while let (Some(left), Some(right)) = (
        blocks
            .iter()
            .position(|block| matches!(block, Block::Empty)),
        blocks
            .iter()
            .rposition(|block| matches!(block, Block::File(_))),
    ) {
        if left >= right {
            break;
        }

        blocks.swap(left, right);
    }
}

fn build_sectors(blocks: &[Block]) -> Vec<Sector> {
    let mut sectors = vec![];
    let chunks = blocks.iter().enumerate().chunk_by(|(_, block)| *block);
    let mut empty_len = 0;
    for (_, mut chunk) in &chunks {
        if let Some((start, block)) = chunk.next() {
            let len = chunk.count() + 1;

            sectors.push(match block {
                Block::File(id) => Sector::File(*id, start, len, empty_len >= len),
                Block::Empty => Sector::Empty(start, len),
            });

            if block == &Block::Empty && empty_len < len {
                empty_len = len;
            }
        }
    }
    sectors
}

fn defragment_file(blocks: &mut [Block]) {
    let mut last_file_id = usize::MAX;
    loop {
        let sectors = build_sectors(blocks);

        let Some(file_sector) = sectors
            .iter()
            .rfind(|sector| matches!(sector, Sector::File(file_id, _, _, true) if *file_id < last_file_id))
        else {
            break;
        };

        if let Sector::File(file_id, file_start, file_len, _) = file_sector {
            let target = sectors
                .iter()
                .find(|sector| matches!(sector, Sector::Empty(start, len) if start < file_start && len >= file_len));

            if let Some(Sector::Empty(empty_start, _)) = target {
                blocks
                    .iter_mut()
                    .skip(*empty_start)
                    .take(*file_len)
                    .for_each(|block| *block = Block::File(*file_id));

                blocks
                    .iter_mut()
                    .skip(*file_start)
                    .take(*file_len)
                    .for_each(|block| *block = Block::Empty);
            }

            last_file_id = *file_id;
        }
    }
}

fn checksum(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .enumerate()
        .fold(0, |sum, (index, block)| match block {
            Block::File(id) => sum + index * id,
            Block::Empty => sum,
        })
}

fn solve1(blocks: &mut [Block]) -> usize {
    defragment_block(blocks);
    checksum(blocks)
}

fn solve2(blocks: &mut [Block]) -> usize {
    defragment_file(blocks);
    checksum(blocks)
}

fn main() {
    let blocks = read_input("test.txt");
    let answer = solve1(&mut blocks.clone());
    println!("Part 1 answer is: {answer}");

    let answer = solve2(&mut blocks.clone());
    println!("Part 2 answer is: {answer}");
}
