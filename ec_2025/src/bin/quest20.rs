use std::{collections::VecDeque, time::Instant};

use itertools::Itertools;
use rustc_hash::FxHashSet;

const INPUT_PART1: &str = include_str!("inputs/quest20-1.txt");
const INPUT_PART2: &str = include_str!("inputs/quest20-2.txt");
const INPUT_PART3: &str = include_str!("inputs/quest20-3.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum CellType {
    Trampoline,
    Empty,
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            'T' | 'E' | 'S' => Self::Trampoline,
            _ => Self::Empty,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Edge {
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Cell {
    typ: CellType,
    edge: Edge,
}

impl Cell {
    fn new(c: char, dir: Edge) -> Self {
        Self {
            typ: c.into(),
            edge: dir,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    start: (usize, usize),
    end: (usize, usize),
    rotate: bool,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let cells = value
            .trim()
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.chars()
                    .skip(row)
                    .take_while(|c| *c != '.')
                    .enumerate()
                    .zip([Edge::Up, Edge::Down].iter().cycle())
                    .map(|((col, c), &dir)| {
                        if c == 'S' {
                            start = (row, col);
                        } else if c == 'E' {
                            end = (row, col);
                        }
                        Cell::new(c, dir)
                    })
                    .collect()
            })
            .collect();
        Grid {
            cells,
            start,
            end,
            rotate: false,
        }
    }
}

impl Grid {
    fn adjacent_trampolines(&self) -> usize {
        // Go through all the rows but the last and look for neighbors to the right and below (if one can be below).
        self.cells
            .iter()
            .enumerate()
            .take(self.cells.len() - 1)
            .flat_map(|(r, row)| {
                row.iter()
                    .tuple_windows()
                    .enumerate()
                    .filter(|(_, (cell, _))| cell.typ != CellType::Empty)
                    .map(move |(c, (cell, next))| {
                        // If there is a trampoline next to use, it's a match.
                        let mut adjacent = 0;
                        if next.typ == CellType::Trampoline {
                            adjacent += 1;
                        }

                        // If we are up and something below us is down, it's also a match.
                        if cell.edge == Edge::Down
                            && self.cells[r + 1][c - 1].typ == CellType::Trampoline
                            && self.cells[r + 1][c - 1].edge == Edge::Up
                        {
                            adjacent += 1;
                        }
                        adjacent
                    })
            })
            .sum()
    }
}

fn parse(input: &str) -> Grid {
    Grid::from(input)
}

fn p1(input: &str) -> usize {
    parse(input).adjacent_trampolines()
}

impl Grid {
    // Calculate valid neighbors from given position. NOTE: If `grid.rotate == true` we'll rotate
    // row/col for p3 and add that position as a neighbor if it's a Trampoline.
    fn neighbors(&self, (mut row, mut col): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];

        if self.rotate {
            // First, rotate our current position
            (row, col) = self.rotate_position((row, col));

            // Check if we landed on a trampoline
            if self.cells[row][col].typ == CellType::Trampoline {
                // We can stay at the rotated position
                neighbors.push((row, col));
            }
        }

        // Check left.
        if col > 0 && self.cells[row][col - 1].typ == CellType::Trampoline {
            neighbors.push((row, col - 1));
        }

        // Check right.
        if col < self.cells[row].len() - 1 && self.cells[row][col + 1].typ == CellType::Trampoline {
            neighbors.push((row, col + 1));
        }

        // Check Up/Down.
        match self.cells[row][col].edge {
            Edge::Up => {
                if row > 0 && self.cells[row - 1][col + 1].typ == CellType::Trampoline {
                    neighbors.push((row - 1, col + 1))
                }
            }
            Edge::Down => {
                if row < self.cells.len() - 1
                    && self.cells[row + 1][col - 1].typ == CellType::Trampoline
                {
                    neighbors.push((row + 1, col - 1))
                }
            }
        }

        neighbors
    }
}

fn bfs(grid: &Grid) -> usize {
    let mut frontier = VecDeque::new();
    frontier.push_back((grid.start, 0));

    let mut seen = FxHashSet::default();
    seen.insert(grid.start);

    while let Some(((r, c), cost)) = frontier.pop_front() {
        if (r, c) == grid.end {
            return cost;
        }

        for neighbor in grid.neighbors((r, c)) {
            if seen.insert(neighbor) {
                frontier.push_back((neighbor, cost + 1));
            }
        }
    }

    unreachable!()
}

fn p2(input: &str) -> usize {
    let grid = parse(input);
    bfs(&grid)
}

impl Grid {
    // See: https://en.wikipedia.org/wiki/Barycentric_coordinate_system
    fn rotate_position(&self, (row, col): (usize, usize)) -> (usize, usize) {
        let new_row = (self.cells.len() - 1) - row - col.div_ceil(2);
        let new_col = if col.is_multiple_of(2) {
            2 * row
        } else {
            (2 * row) + 1
        };
        (new_row, new_col)
    }
}

fn p3(input: &str) -> usize {
    let mut grid = parse(input);
    grid.rotate = true;
    bfs(&grid)
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT_PART1);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT_PART2);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p3(INPUT_PART3);
    println!("p3 {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "T#TTT###T##\n.##TT#TT##.\n..T###T#T..\n...##TT#...\n....T##....\n.....#.....";
        assert_eq!(p1(input), 7);
    }

    #[test]
    fn test_p2() {
        let input = "TTTTTTTTTTTTTTTTT\n.TTTT#T#T#TTTTTT.\n..TT#TTTETT#TTT..\n...TT#T#TTT#TT...\n....TTT#T#TTT....\n.....TTTTTT#.....\n......TT#TT......\n.......#TT.......\n........S........";
        assert_eq!(p2(input), 32);
    }

    #[test]
    fn test_p3() {
        let input = "T####T#TTT##T##T#T#\n.T#####TTTT##TTT##.\n..TTTT#T###TTTT#T..\n...T#TTT#ETTTT##...\n....#TT##T#T##T....\n.....#TT####T#.....\n......T#TT#T#......\n.......T#TTT.......\n........TT#........\n.........S.........";
        assert_eq!(p3(input), 23);
    }
}
