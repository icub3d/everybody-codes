use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

#[derive(Debug, Eq, PartialEq)]
struct Input {
    a: usize,
    b: usize,
    c: usize,
    x: usize,
    y: usize,
    z: usize,
    m: usize,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let mut pp = value
            .split_whitespace()
            .map(|c| c.split_once('=').unwrap().1.parse::<usize>().unwrap());
        Self {
            a: pp.next().unwrap(),
            b: pp.next().unwrap(),
            c: pp.next().unwrap(),
            x: pp.next().unwrap(),
            y: pp.next().unwrap(),
            z: pp.next().unwrap(),
            m: pp.next().unwrap(),
        }
    }
}

const INPUT_PART1: &str = include_str!("inputs/quest01-1.txt");
type InputPart1<'a> = Vec<Input>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input.trim().lines().map(Input::from).collect()
}

fn eni(n: usize, exp: usize, m: usize) -> usize {
    let mut rems = VecDeque::new();

    let mut cur = 1;

    for _ in 0..exp {
        cur *= n;
        cur %= m;
        rems.push_front(cur);
    }

    rems.iter()
        .map(|i| i.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn p1(input: &InputPart1) -> usize {
    input
        .iter()
        .map(|i| eni(i.a, i.x, i.m) + eni(i.b, i.y, i.m) + eni(i.c, i.z, i.m))
        .max()
        .unwrap()
}

fn eni2(n: usize, exp: usize, m: usize) -> usize {
    let mut path = Vec::with_capacity(m + 1);
    let mut visited = HashMap::with_capacity(m + 1);
    let mut score = 1;

    // Look for a cycle
    let (start, end) = loop {
        // If we've seen this before, we have a cycle.
        if let Some(&start) = visited.get(&score) {
            break (start, path.len() - start);
        }
        visited.insert(score, path.len());
        path.push(score);
        score = (score * n) % m;
    };

    // Get up to the last 5 values in the cycle and add them to our buffer
    let mut buf = String::new();
    for p in 0..exp.min(5) {
        let pos = (exp - p - start) % end;
        buf.push_str(&path[start + pos].to_string());
    }

    // Return it as a usize
    buf.parse::<usize>().unwrap()
}

fn p2(input: &InputPart2) -> usize {
    input
        .iter()
        .map(|i| eni2(i.a, i.x, i.m) + eni2(i.b, i.y, i.m) + eni2(i.c, i.z, i.m))
        .max()
        .unwrap()
}

fn eni3(n: usize, exp: usize, m: usize) -> usize {
    let mut path = Vec::with_capacity(m + 1);
    let mut visited = HashMap::new();
    let mut score = 1;

    // Look for a cycle.
    let (start, end) = loop {
        if let Some(&start) = visited.get(&score) {
            break (start, path.len() - start);
        }
        visited.insert(score, path.len());
        path.push(score);
        score = (score * n) % m;
    };

    let len = exp - start + 1;
    let cycles = len / end;
    let remainder = len % end;

    // pre-cycle + cycles * sum + remainder - 1 (we added one to start)
    path[0..start].iter().sum::<usize>()
        + cycles * path[start..start + end].iter().sum::<usize>()
        + path[start..start + remainder].iter().sum::<usize>()
        - 1
}

fn p3(input: &InputPart3) -> usize {
    input
        .iter()
        .map(|i| eni3(i.a, i.x, i.m) + eni3(i.b, i.y, i.m) + eni3(i.c, i.z, i.m))
        .max()
        .unwrap()
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

const INPUT_PART2: &str = include_str!("inputs/quest01-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest01-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = parse_input_part1(
            "A=4 B=4 C=6 X=3 Y=4 Z=5 M=11\nA=8 B=4 C=7 X=8 Y=4 Z=6 M=12\nA=2 B=8 C=6 X=2 Y=4 Z=5 M=13\nA=5 B=9 C=6 X=8 Y=6 Z=8 M=14\nA=5 B=9 C=7 X=6 Y=6 Z=8 M=15\nA=8 B=8 C=8 X=6 Y=9 Z=6 M=16",
        );
        assert_eq!(p1(&input), 11611972920);
    }

    #[test]
    fn test_p2() {
        let input = parse_input_part2(
            "A=4 B=4 C=6 X=3 Y=14 Z=15 M=11\nA=8 B=4 C=7 X=8 Y=14 Z=16 M=12\nA=2 B=8 C=6 X=2 Y=14 Z=15 M=13\nA=5 B=9 C=6 X=8 Y=16 Z=18 M=14\nA=5 B=9 C=7 X=6 Y=16 Z=18 M=15\nA=8 B=8 C=8 X=6 Y=19 Z=16 M=16",
        );
        assert_eq!(p2(&input), 11051340);
    }

    #[test]
    fn test_p2_big() {
        let input = parse_input_part2(
            "A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188\nA=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107\nA=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102\nA=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196\nA=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145",
        );
        assert_eq!(p2(&input), 1507702060886);
    }

    #[test]
    fn test_p3() {
        let input = parse_input_part3(
            "A=4 B=4 C=6 X=3000 Y=14000 Z=15000 M=110\nA=8 B=4 C=7 X=8000 Y=14000 Z=16000 M=120\nA=2 B=8 C=6 X=2000 Y=14000 Z=15000 M=130\nA=5 B=9 C=6 X=8000 Y=16000 Z=18000 M=140\nA=5 B=9 C=7 X=6000 Y=16000 Z=18000 M=150\nA=8 B=8 C=8 X=6000 Y=19000 Z=16000 M=160",
        );
        assert_eq!(p3(&input), 3279640);
    }

    #[test]
    fn test_p3_big() {
        let input = parse_input_part3(
            "A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188\nA=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107\nA=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102\nA=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196\nA=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145",
        );
        assert_eq!(p3(&input), 7276515438396);
    }
}
