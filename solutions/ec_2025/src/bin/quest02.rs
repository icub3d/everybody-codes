use std::{
    fmt::Display,
    ops::{Add, AddAssign, DivAssign, MulAssign, RangeInclusive},
    time::Instant,
};

use itertools::Itertools;

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
struct Complex {
    // LOL - I was using i32, the numbers get too big!
    x: isize,
    y: isize,
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Complex {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

// LOL - self.x = ...; self.y = ...; messed me up here.
// [self.x,self.y] * [rhs.x,rhs.y] = [self.x * rhs.x - self.y * rhs.y, self.x * rhs.y + self.y * rhs.x]
impl MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, rhs: Complex) {
        let x = self.x * rhs.x - self.y * rhs.y;
        let y = self.x * rhs.y + self.y * rhs.x;
        self.x = x;
        self.y = y;
    }
}

impl DivAssign<Complex> for Complex {
    fn div_assign(&mut self, rhs: Complex) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Complex> for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

const INPUT_PART1: &str = include_str!("inputs/quest02-1.txt");
type InputPart1<'a> = Complex;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .trim()
        .trim_start_matches("A=[")
        .trim_end_matches("]")
        .split_once(',')
        .map(|(r, i)| Complex {
            x: r.parse::<isize>().unwrap(),
            y: i.parse::<isize>().unwrap(),
        })
        .unwrap()
}

fn p1(a: InputPart1) -> String {
    let mut r = Complex::new(0, 0);
    (0..3).for_each(|_| {
        r *= r;
        r /= Complex::new(10, 10);
        r += a;
    });
    format!("{}", r)
}

const VALID_RANGE: RangeInclusive<isize> = -1_000_000..=1_000_000;

fn cycle(a: Complex, x: isize, y: isize) -> bool {
    let p = a + Complex::new(x, y);
    let mut r = Complex::new(0, 0);
    (0..100).all(|_| {
        r *= r;
        r /= Complex::new(100_000, 100_000);
        r += p;
        VALID_RANGE.contains(&r.x) && VALID_RANGE.contains(&r.y)
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
        .cartesian_product(0..=1_000)
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

