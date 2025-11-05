use std::time::Instant;

use itertools::Itertools;
use rustc_hash::FxHashMap;

const INPUT_PART1: &str = include_str!("inputs/quest03-1.txt");
type InputPart1<'a> = Vec<isize>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .trim()
        .split(',')
        .map(|p| p.parse::<isize>().unwrap())
        .collect()
}

fn p1(input: &InputPart1) -> isize {
    input.iter().sorted().dedup().sum()
}

fn p2(input: &InputPart2) -> isize {
    input.iter().sorted().dedup().take(20).sum()
}

fn p3(input: &InputPart3) -> isize {
    let counts = input.iter().fold(FxHashMap::default(), |mut acc, i| {
        *acc.entry(i).or_default() += 1;
        acc
    });
    *counts.values().max().unwrap()
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

const INPUT_PART2: &str = include_str!("inputs/quest03-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest03-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

