use std::{collections::VecDeque, time::Instant};

use rustc_hash::{FxHashMap, FxHashSet};

const INPUT_PART1: &str = include_str!("inputs/quest05-1.txt");
type InputPart1<'a> = Vec<VecDeque<usize>>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .trim()
        .lines()
        .fold(vec![VecDeque::new(); 4], |mut acc, l| {
            let pp = l
                .split_whitespace()
                .map(|p| p.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (0..4).for_each(|i| acc[i].push_back(pp[i]));
            acc
        })
}

fn step(input: &mut [VecDeque<usize>], turn: usize) -> usize {
    // turn is zero based.
    let turn = turn % 4;
    let clapper = input[turn].pop_front().unwrap();
    let column = (turn + 1) % 4;
    let circuit_len = input[column].len() * 2;
    let pos = (clapper - 1) % circuit_len;
    let pos = pos.min(circuit_len - pos);
    input[column].insert(pos, clapper);
    input.iter().map(|v| v[0]).fold(0usize, |acc, i| {
        acc * 10usize.pow((i as f64).log10().floor() as u32 + 1) + i
    })
}

fn p1(input: &mut InputPart1) -> usize {
    (0..10).map(|i| step(input, i)).last().unwrap()
}

const INPUT_PART2: &str = include_str!("inputs/quest05-2.txt");
type InputPart2<'a> = Vec<VecDeque<usize>>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

fn p2(input: &mut InputPart2) -> usize {
    let mut shouts: FxHashMap<usize, usize> = FxHashMap::default();
    (0..)
        .find_map(|i| {
            let shout = step(input, i);
            let s = shouts.entry(shout).or_default();
            *s += 1;
            if *s == 2024 {
                Some(shout * (i + 1))
            } else {
                None
            }
        })
        .unwrap()
}

const INPUT_PART3: &str = include_str!("inputs/quest05-3.txt");
type InputPart3<'a> = Vec<VecDeque<usize>>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

fn str(input: &InputPart3) -> String {
    input
        .iter()
        .map(|r| {
            r.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn p3(input: &mut InputPart3) -> usize {
    let mut states: FxHashSet<String> = FxHashSet::default();
    states.insert(str(input));
    let mut highest = 0;
    for i in 0.. {
        let shout = step(input, i);
        highest = highest.max(shout);
        if !states.insert(str(input)) {
            return highest;
        }
    }
    unreachable!()
}

fn main() {
    let now = Instant::now();
    let mut input = parse_input_part1(INPUT_PART1);
    let solution = p1(&mut input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let mut input = parse_input_part2(INPUT_PART2);
    let solution = p2(&mut input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let mut input = parse_input_part3(INPUT_PART3);
    let solution = p3(&mut input);
    println!("p3 {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4";
        let mut input = parse_input_part1(input);
        assert_eq!(
            input,
            vec![
                vec![2, 3, 4, 5],
                vec![3, 4, 5, 2],
                vec![4, 5, 2, 3],
                vec![5, 2, 3, 4]
            ]
        );
        assert_eq!(p1(&mut input), 2323);
    }

    #[test]
    fn test_p2() {
        let input = "2 3 4 5
6 7 8 9";
        let mut input = parse_input_part2(input);
        assert_eq!(p2(&mut input), 50877075);
    }

    #[test]
    fn test_p3() {
        let input = "2 3 4 5
6 7 8 9";
        let mut input = parse_input_part3(input);
        assert_eq!(p3(&mut input), 6584);
    }
}
