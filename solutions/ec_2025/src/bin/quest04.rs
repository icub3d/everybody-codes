use std::time::Instant;

const INPUT_PART1: &str = include_str!("inputs/quest04-1.txt");
type InputPart1<'a> = (usize, Vec<(usize, usize)>, usize);
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    let input = input.trim().lines().collect::<Vec<_>>();

    let first = input[0].parse::<usize>().unwrap();
    let last = input[input.len() - 1].parse::<usize>().unwrap();

    let middle = input[1..input.len() - 1]
        .iter()
        .map(|l| {
            l.split_once('|')
                .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
                .unwrap_or((0, 0))
            // If we don't have |, we don't care because p1 and p2 don't
            // care about middle
        })
        .collect();

    (first, middle, last)
}

fn p1(&(first, _, last): &InputPart1) -> usize {
    first * 2025 / last
}

fn p2(&(first, _, last): &InputPart2) -> usize {
    // Note: This can be simplified with div_ceil, which I found later.
    let n = last * 10_000_000_000_000;
    let d = first;
    // We need to calculate full turns.
    n / d + if !n.is_multiple_of(d) { 1 } else { 0 }
}

fn p3(&(first, ref middle, last): &InputPart3) -> usize {
    middle.iter().fold(first * 100, |acc, (l, r)| acc * r / l) / last
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

const INPUT_PART2: &str = include_str!("inputs/quest04-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest04-3.txt");
type InputPart3<'a> = (usize, Vec<(usize, usize)>, usize);
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

