use std::{ops::Index, time::Instant};

use rustc_hash::FxHashMap;

const INPUT_PART1: &str = include_str!("inputs/quest06-1.txt");
type InputPart1<'a> = Vec<char>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input.trim().chars().collect()
}

fn p1(input: &InputPart1) -> usize {
    input
        .iter()
        .filter(|c| "Aa".contains(**c))
        .fold((0, 0), |(mentors, total), c| match c {
            'A' => (mentors + 1, total),
            'a' => (mentors, total + mentors),
            _ => panic!("oops"),
        })
        .1
}

fn p2(input: &InputPart2) -> usize {
    input
        .iter()
        .scan(FxHashMap::default(), |mentors, c| match c {
            'A' => {
                *mentors.entry('A').or_default() += 1;
                Some(0)
            }
            'B' => {
                *mentors.entry('B').or_default() += 1;
                Some(0)
            }
            'C' => {
                *mentors.entry('C').or_default() += 1;
                Some(0)
            }
            'a' => Some(mentors[&'A']),
            'b' => Some(mentors[&'B']),
            'c' => Some(mentors[&'C']),
            _ => Some(0),
        })
        .sum()
}

#[derive(Default)]
struct Mentors {
    a: usize,
    b: usize,
    c: usize,
}

// This shadows a bit of logic. If it's not (a, b, c), then it will return 0.
impl Index<char> for Mentors {
    type Output = usize;
    fn index(&self, index: char) -> &Self::Output {
        match index {
            'a' => &self.a,
            'b' => &self.b,
            'c' => &self.c,
            _ => &0,
        }
    }
}

// These shadows a bit of logic. If it's not (A, B, C), then it won't change the values.
impl Mentors {
    fn add(&mut self, c: char, v: usize) {
        match c {
            'A' => self.a += v,
            'B' => self.b += v,
            'C' => self.c += v,
            _ => (),
        }
    }

    fn sub(&mut self, c: char, v: usize) {
        match c {
            'A' => self.a -= v,
            'B' => self.b -= v,
            'C' => self.c -= v,
            _ => (),
        }
    }
}

fn p3(input: &InputPart3, window: isize, repeats: usize) -> usize {
    // The algorithm here is a sliding window. Instead of sliding through a "fake" 1000x window
    // though, we can assume each mentor will see a trainee 1_000 times. The one gotcha here are
    // the beginning and end of the array. Since we aren't expanding infinitely, those will repeat
    // one less time. This is because at the beginning and end, those won't be there anymore.
    let len = input.len() as isize;

    // Setup our initial window
    let mut mentors = Mentors::default();
    (-window..=window)
        .map(|i| (i, input[i.rem_euclid(len) as usize]))
        .filter(|(_, c)| "ABC".contains(*c))
        .for_each(|(i, c)| mentors.add(c, if i < 0 { repeats - 1 } else { repeats }));

    // Our running total and our left and right side of the window.
    let mut total = 0;
    let mut left = -window;
    let mut right = window;

    // Slide through.
    for cur in 0..input.len() {
        // Update our total
        total += mentors[input[cur]];

        // shift the sliding window.
        right += 1;
        mentors.add(
            input[right.rem_euclid(len) as usize],
            if right >= len { repeats - 1 } else { repeats },
        );
        mentors.sub(
            input[left.rem_euclid(len) as usize],
            if left < 0 { repeats - 1 } else { repeats },
        );
        left += 1;
    }

    total
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
    let solution = p3(&input, 1_000, 1_000);
    println!("p3 {:?} {}", now.elapsed(), solution);
}

const INPUT_PART2: &str = include_str!("inputs/quest06-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest06-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}
