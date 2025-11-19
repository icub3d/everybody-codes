use std::{collections::VecDeque, time::Instant};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefMutIterator, ParallelBridge, ParallelIterator};

// Using a BitMask to track barrels was faster than FxHashMap and Vec<bool>.
#[derive(Clone)]
struct BitMask {
    data: Vec<u128>,
    cols: usize,
}

impl BitMask {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0; (rows * cols).div_ceil(128)],
            cols,
        }
    }

    #[inline]
    fn index(&self, r: isize, c: isize) -> usize {
        r as usize * self.cols + c as usize
    }

    #[inline]
    fn set(&mut self, r: isize, c: isize) -> bool {
        let idx = self.index(r, c);
        let word = idx / 128;
        let mask = 1u128 << (idx % 128);
        let was_set = self.data[word] & mask != 0;
        self.data[word] |= mask;
        !was_set
    }

    fn count(&self) -> usize {
        self.data.iter().map(|w| w.count_ones() as usize).sum()
    }

    fn subtract(&mut self, other: &BitMask) {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a &= !*b;
        }
    }
}

const INPUT_PART1: &str = include_str!("inputs/quest12-1.txt");
type InputPart1<'a> = Vec<Vec<usize>>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

// Do a BFS to find all neighboring barrels that would explode if the given barrels are ignited.
fn bfs(input: &InputPart3, barrels: &[(isize, isize)]) -> BitMask {
    let (max_row, max_col) = (input.len(), input[0].len());
    let mut seen = BitMask::new(max_row, max_col);

    let mut frontier = VecDeque::with_capacity(max_row * max_col / 4);
    for &(r, c) in barrels {
        seen.set(r, c);
        frontier.push_back((r, c));
    }

    while let Some((r, c)) = frontier.pop_front() {
        for (dr, dc) in NEIGHBORS.iter() {
            let (nr, nc) = (r + dr, c + dc);
            if nr >= 0
                && nr < max_row as isize
                && nc >= 0
                && nc < max_col as isize
                && input[nr as usize][nc as usize] <= input[r as usize][c as usize]
                && seen.set(nr, nc)
            {
                frontier.push_back((nr, nc));
            }
        }
    }
    seen
}

fn p1(input: &InputPart1) -> usize {
    bfs(input, &[(0, 0)]).count()
}

fn p2(input: &InputPart2) -> usize {
    bfs(
        input,
        &[
            (0, 0),
            ((input.len() - 1) as isize, (input[0].len() - 1) as isize),
        ],
    )
    .count()
}

fn p3(input: &InputPart3) -> usize {
    // BFS all positions in parallel
    let mut results: Vec<BitMask> = (0..input.len() as isize)
        .cartesian_product(0..input[0].len() as isize)
        .par_bridge()
        .map(|(r, c)| bfs(input, &[(r, c)]))
        .collect();

    // Instead of sorting O(n log n), we can just max_by_key O(n)
    // We then pull it out using swap_remove.
    // We can do this for the second as well

    // Find largest
    let largest_index = results
        .iter()
        .enumerate()
        .max_by_key(|(_, m)| m.count())
        .unwrap()
        .0;
    let largest = results.swap_remove(largest_index);
    let first_size = largest.count();

    // Remove overlap from all remaining (par_iter_mut).
    results.par_iter_mut().for_each(|mask| {
        mask.subtract(&largest);
    });

    // Find second largest
    let second_index = results
        .iter()
        .enumerate()
        .max_by_key(|(_, m)| m.count())
        .unwrap()
        .0;
    let second = results.swap_remove(second_index);
    let second_size = second.count();

    // Remove overlap from remaining
    results.par_iter_mut().for_each(|mask| {
        mask.subtract(&second);
    });

    // Find third largest
    let third_size = results.iter().map(|m| m.count()).max().unwrap();

    first_size + second_size + third_size
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

const INPUT_PART2: &str = include_str!("inputs/quest12-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest12-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = parse_input_part1(INPUT_PART1);
        assert_eq!(input, input);
    }

    #[test]
    fn test_p2() {
        let input = parse_input_part2(INPUT_PART2);
        assert_eq!(input, input);
    }

    #[test]
    fn test_p3() {
        let input = parse_input_part3(INPUT_PART3);
        assert_eq!(input, input);
    }
}
