use std::ops::RangeInclusive;
use std::time::Instant;

use itertools::Itertools;
use num::complex::Complex;
use rayon::prelude::*;

const INPUT_PART1: &str = include_str!("inputs/quest02-1.txt");
type InputPart1<'a> = Complex<isize>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .trim()
        .trim_start_matches("A=[")
        .trim_end_matches("]")
        .split_once(',')
        .map(|(r, i)| Complex::new(r.parse::<isize>().unwrap(), i.parse::<isize>().unwrap()))
        .unwrap()
}

fn p1(a: InputPart1) -> String {
    let mut r = Complex::new(0, 0);
    (0..3).for_each(|_| {
        r *= r;
        r /= 10;
        r += a;
    });
    format!("[{},{}]", r.re, r.im)
}

const VALID_RANGE: RangeInclusive<isize> = -1_000_000..=1_000_000;

fn cycle(a: Complex<isize>, x: isize, y: isize) -> bool {
    let p = a + Complex::new(x, y);
    let mut r = Complex::new(0, 0);
    (0..100).all(|_| {
        r *= r;
        r /= 100_000;
        r += p;
        VALID_RANGE.contains(&r.re) && VALID_RANGE.contains(&r.im)
    })
}

fn p2(a: InputPart2) -> usize {
    (0..=1000)
        .step_by(10)
        .cartesian_product((0..=1_000).step_by(10))
        .filter(|(x, y)| cycle(a, *x, *y))
        .count()
}

fn p3(a: InputPart3) -> usize {
    (0..=1000)
        .into_par_iter()
        .flat_map(|x| (0..=1000).into_par_iter().map(move |y| (x, y)))
        .filter(|(x, y)| cycle(a, *x, *y))
        .count()
}

fn main() {
    let now = Instant::now();
    let input = parse_input_part1(INPUT_PART1);
    let solution = p1(input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part2(INPUT_PART2);
    let solution = p2(input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part3(INPUT_PART3);
    let solution = p3(input);
    println!("p3 {:?} {}", now.elapsed(), solution);
}

const INPUT_PART2: &str = include_str!("inputs/quest02-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest02-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}
