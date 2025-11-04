use std::time::Instant;

use itertools::Itertools;

const INPUT1: &str = include_str!("inputs/quest01-1.txt");
type Input1<'a> = &'a str;
fn parse_input1(input: &'_ str) -> Input1<'_> {
    input.trim()
}

fn enemy_cost(c: char) -> usize {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => 0,
    }
}

fn p1(input: &Input1) -> usize {
    input.chars().map(enemy_cost).sum()
}

const INPUT2: &str = include_str!("inputs/quest01-2.txt");
type Input2<'a> = &'a str;
fn parse_input2(input: &'_ str) -> Input2<'_> {
    input.trim()
}

fn p2_cost((l, r): (char, char)) -> usize {
    let grouped = if l == 'x' || r == 'x' { 0 } else { 2 };
    enemy_cost(l) + enemy_cost(r) + grouped
}

fn p2(input: &Input2) -> usize {
    input.chars().tuples().map(p2_cost).sum()
}

const INPUT3: &str = include_str!("inputs/quest01-3.txt");
type Input3<'a> = &'a str;
fn parse_input3(input: &'_ str) -> Input3<'_> {
    input.trim()
}

fn p3_cost((l, m, r): (char, char, char)) -> usize {
    let xs = [l, m, r].iter().filter(|c| **c == 'x').count();
    let grouped = match xs {
        2 => 0,
        1 => 2,
        0 => 6,
        _ => 0,
    };

    enemy_cost(l) + enemy_cost(m) + enemy_cost(r) + grouped
}

fn p3(input: &Input3) -> usize {
    input.chars().tuples::<(_, _, _)>().map(p3_cost).sum()
}

fn main() {
    let now = Instant::now();
    let input = parse_input1(INPUT1);
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input2(INPUT2);
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input3(INPUT3);
    let solution = p3(&input);
    println!("p3 {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "ABBAC";
        assert_eq!(p1(&input), 5);
    }

    #[test]
    fn test_p3() {
        let input = "xBxAAABCDxCC";
        assert_eq!(p3(&input), 30);
    }
}
