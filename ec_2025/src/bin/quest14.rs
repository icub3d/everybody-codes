use std::time::Instant;

use rustc_hash::FxHashMap;

const INPUT_PART1: &str = include_str!("inputs/quest14-1.txt");
type InputPart1 = Grid;
fn parse_input_part1(input: &str) -> InputPart1 {
    Grid::from(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Active,
    Inactive,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Active,
            _ => Tile::Inactive,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self {
            grid: value
                .trim()
                .lines()
                .map(|l| l.chars().map(Tile::from).collect())
                .collect(),
        }
    }
}

impl Grid {
    // used for p3 to make an empty grid.
    fn new(size: usize) -> Self {
        Self {
            grid: vec![vec![Tile::Inactive; size]; size],
        }
    }

    fn next(&self) -> Self {
        let mut next = self.grid.clone();

        // Go through all the current values and update the next values.
        for (r, line) in self.grid.iter().enumerate() {
            for (c, &v) in line.iter().enumerate() {
                let neighbors = Self::diagonals(&self.grid, r as isize, c as isize);
                next[r][c] = match (v, neighbors.is_multiple_of(2)) {
                    (Tile::Active, true) => Tile::Inactive,
                    (Tile::Inactive, true) => Tile::Active,
                    _ => v,
                };
            }
        }

        Self { grid: next }
    }

    fn value(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|&&v| v == Tile::Active)
            .count()
    }

    const DIAGONALS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    fn diagonals(grid: &[Vec<Tile>], r: isize, c: isize) -> usize {
        // diagonals need to be in the grid and have a value of Active.
        Self::DIAGONALS
            .iter()
            .filter(|&&(dr, dc)| {
                let (nr, nc) = (r + dr, c + dc);
                nr >= 0
                    && nc >= 0
                    && (nr as usize) < grid.len()
                    && (nc as usize) < grid[0].len()
                    && grid[nr as usize][nc as usize] == Tile::Active
            })
            .count()
    }

    fn center_matches(&self, center: &[Vec<Tile>]) -> bool {
        // Figure out where the center would start.
        let (sr, sc) = (
            (self.grid.len() - center.len()) / 2,
            (self.grid[0].len() - center[0].len()) / 2,
        );

        // Ensure all values are equal.
        center
            .iter()
            .enumerate()
            .flat_map(|(r, l)| l.iter().enumerate().map(move |(c, v)| (r, c, v)))
            .all(|(r, c, v)| self.grid[sr + r][sc + c] == *v)
    }
}

// We can do the same thing for p1 and p2.
fn simulate(input: &Grid, rounds: usize) -> usize {
    (0..rounds)
        .scan(input.clone(), |grid, _| {
            *grid = grid.next();
            Some(grid.value())
        })
        .sum()
}

fn p1(input: &InputPart1) -> usize {
    simulate(input, 10)
}

fn p2(input: &InputPart2) -> usize {
    simulate(input, 2025)
}

struct CycleDetector<T> {
    seen: FxHashMap<T, usize>, // Previously seen states.
    prefix_sums: Vec<usize>,   // running totals
    rounds: usize,             // total rounds
}

impl<T: std::hash::Hash + Eq> CycleDetector<T> {
    fn new(rounds: usize) -> Self {
        Self {
            seen: FxHashMap::default(),
            prefix_sums: vec![0],
            rounds,
        }
    }

    fn step(&mut self, state: T, value: usize) -> Option<usize> {
        // Update our state.
        let round = self.prefix_sums.len();
        self.prefix_sums
            .push(self.prefix_sums.last().unwrap() + value);

        // If we've seen this state before, we can now calculate the total.
        if let Some(start) = self.seen.insert(state, round) {
            let len = round - start;
            let remaining = self.rounds - round;
            let sum = self.prefix_sums[round] - self.prefix_sums[start];

            // total so far + cycles * cycle_value + leftover
            return Some(
                self.prefix_sums[round]
                    + (remaining / len) * sum
                    + (self.prefix_sums[start + remaining % len] - self.prefix_sums[start]),
            );
        }

        // Shouldn't get here but if we don't get a cycle, it will just be the last value.
        (round >= self.rounds).then(|| self.prefix_sums[self.rounds])
    }
}

fn p3(center: &InputPart3) -> usize {
    const TOTAL_ROUNDS: usize = 1_000_000_000;
    let mut grid = Grid::new(34);
    let mut detector = CycleDetector::new(TOTAL_ROUNDS);

    loop {
        // Update our state and it's value.
        grid = grid.next();
        let value = if grid.center_matches(&center.grid) {
            grid.value()
        } else {
            0
        };

        // Use the cycle detector and return when we find one.
        if let Some(total) = detector.step(grid.clone(), value) {
            return total;
        }
    }
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

const INPUT_PART2: &str = include_str!("inputs/quest14-2.txt");
type InputPart2 = InputPart1;
fn parse_input_part2(input: &str) -> InputPart2 {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest14-3.txt");
type InputPart3 = InputPart1;
fn parse_input_part3(input: &str) -> InputPart3 {
    parse_input_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = parse_input_part1(".#.##.\n##..#.\n..##.#\n.#.##.\n.###..\n###.##");
        assert_eq!(p1(&input), 200);
    }

    #[test]
    fn test_p2() {
        let input = parse_input_part2(".#.##.\n##..#.\n..##.#\n.#.##.\n.###..\n###.##");
        assert_eq!(p2(&input), 39349);
    }

    #[test]
    fn test_p3() {
        let input = parse_input_part3(
            "#......#\n..#..#..\n.##..##.\n...##...\n...##...\n.##..##.\n..#..#..\n#......#",
        );
        assert_eq!(p3(&input), 278388552);
    }
}
