use std::{
    cmp::Ordering::{Equal, Greater, Less},
    time::Instant,
};

const INPUT_PART1: &str = include_str!("inputs/quest05-1.txt");
type InputPart1<'a> = Vec<(usize, Vec<usize>)>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .lines()
        .map(|l| {
            l.split_once(':')
                .map(|(l, r)| {
                    (
                        l.parse::<usize>().unwrap(),
                        r.split(',').map(|n| n.parse::<usize>().unwrap()).collect(),
                    )
                })
                .unwrap()
        })
        .collect()
}

#[derive(Eq)]
struct Fishbone {
    id: usize,
    nodes: Vec<Segment>,
}

impl PartialEq for Fishbone {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl PartialOrd for Fishbone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fishbone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.quality().cmp(&other.quality()) {
            Less => Less,
            Greater => Greater,
            Equal => self
                .numbers()
                .cmp(&other.numbers())
                .then(self.id.cmp(&other.id)),
        }
    }
}

impl Fishbone {
    // Create a new fishbone from the given values.
    fn new((id, values): (usize, &[usize])) -> Self {
        let mut fishbone = Fishbone {
            id,
            nodes: Vec::new(),
        };
        values.iter().for_each(|v| fishbone.insert(*v));
        fishbone
    }

    // Get the "numbers" for all the nodes for p3
    fn numbers(&self) -> Vec<usize> {
        self.nodes.iter().map(|n| n.number()).collect()
    }

    // Get the quality of the fishbone.
    fn quality(&self) -> usize {
        // This works because all the numbers are 1 digit. I originally used a concat for String
        // and then parsed. You could also do the log trick to find the number of digits to
        // multiply.
        self.nodes
            .iter()
            .map(|n| n.middle)
            .fold(0, |acc, i| 10 * acc + i)
    }

    fn insert(&mut self, value: usize) {
        // try to place it
        for fishbone in self.nodes.iter_mut() {
            if fishbone.try_place(value) {
                return;
            }
        }

        // If we didn't find one, we simply start a new one at the end.
        self.nodes.push(Segment::new(value));
    }
}

#[derive(PartialEq, Eq, Default)]
struct Segment {
    left: Option<usize>,
    middle: usize,
    right: Option<usize>,
}

impl Segment {
    // Create a new node with value in the middle
    fn new(value: usize) -> Self {
        Self {
            left: None,
            middle: value,
            right: None,
        }
    }

    // Return the "number" of the node for p3.
    fn number(&self) -> usize {
        match (self.left, self.right) {
            (Some(l), Some(r)) => l * 100 + self.middle * 10 + r,
            (None, Some(r)) => self.middle * 10 + r,
            (Some(l), None) => l * 10 + self.middle,
            _ => self.middle,
        }
    }

    fn try_place(&mut self, value: usize) -> bool {
        // The logic here is to try and place it based on where it should go. If we can't return
        // false.
        match value.cmp(&self.middle) {
            Less => match self.left {
                Some(_) => false,
                None => {
                    self.left = Some(value);
                    true
                }
            },
            Greater => match self.right {
                Some(_) => false,
                None => {
                    self.right = Some(value);
                    true
                }
            },
            Equal => false,
        }
    }
}

fn p1(input: &InputPart1) -> usize {
    let (id, value) = (input[0].0, &input[0].1);
    Fishbone::new((id, value)).quality()
}

fn p2(input: &InputPart2) -> usize {
    // Get all the fishbones
    let mut fishbones = input
        .iter()
        .map(|(id, values)| Fishbone::new((*id, values)).quality())
        .collect::<Vec<_>>();

    // Sort and do a diff of first and last.
    fishbones.sort();
    fishbones[fishbones.len() - 1] - fishbones[0]
}

fn p3(input: &InputPart3) -> usize {
    // Get all the fishbones
    let mut fishbones = input
        .iter()
        .map(|(id, values)| Fishbone::new((*id, values)))
        .collect::<Vec<_>>();

    // Reverse sort
    fishbones.sort_by(|a, b| b.cmp(a));

    // Calculate checksum
    fishbones
        .iter()
        .enumerate()
        .map(|(i, f)| f.id * (i + 1))
        .sum()
}

fn main() {
    let now = Instant::now();
    let input = parse_input_part1(INPUT_PART1);
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part2(INPUT_PART2);
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part3(INPUT_PART3);
    let solution = p3(&input);
    println!("p3 {:?} {}", now.elapsed(), solution);
}

const INPUT_PART2: &str = include_str!("inputs/quest05-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest05-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}
