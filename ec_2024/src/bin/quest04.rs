use std::time::Instant;

const INPUT_PART1: &str = include_str!("inputs/quest04-1.txt");
type InputPart1<'a> = Vec<usize>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

fn p1(input: &InputPart1) -> usize {
    let min = input.iter().min().unwrap();
    input.iter().map(|i| *i - min).sum()
}

const INPUT_PART2: &str = include_str!("inputs/quest04-2.txt");
type InputPart2<'a> = Vec<usize>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

fn p2(input: &InputPart2) -> usize {
    let min = input.iter().min().unwrap();
    input.iter().map(|i| *i - min).sum()
}

const INPUT_PART3: &str = include_str!("inputs/quest04-3.txt");
type InputPart3<'a> = Vec<isize>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    input.lines().map(|l| l.parse::<isize>().unwrap()).collect()
}

fn p3(input: &InputPart3) -> isize {
    let mut input = input.clone();
    input.sort();
    let median = input[input.len() / 2];
    input.iter().map(|i| (*i - median).abs()).sum::<isize>()
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
