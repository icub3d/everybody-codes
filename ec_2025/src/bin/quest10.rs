use std::{collections::VecDeque, ops::Add, time::Instant};

use rustc_hash::{FxHashMap, FxHashSet};

// We just collect all the information from the input that we'll need to solve the puzzles.
const INPUT_PART1: &str = include_str!("inputs/quest10-1.txt");
type InputPart1<'a> = (Square, Vec<Square>, FxHashSet<Square>, isize, isize);
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    let input = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut dragon = Square::new(0, 0);
    let mut sheep = vec![];
    let mut hideouts = FxHashSet::default();

    for (row, line) in input.iter().enumerate() {
        for (column, c) in line.iter().enumerate() {
            match c {
                'D' => {
                    dragon = Square::new(column, row);
                }
                '#' => {
                    hideouts.insert(Square::new(column, row));
                }
                'S' => {
                    sheep.push(Square::new(column, row));
                }
                _ => (),
            }
        }
    }
    (
        dragon,
        sheep,
        hideouts,
        input.len() as isize,
        input[0].len() as isize,
    )
}

#[derive(Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Square {
    column: isize,
    row: isize,
}

impl Square {
    fn new(column: usize, row: usize) -> Self {
        Square {
            column: column as isize,
            row: row as isize,
        }
    }

    const KNIGHT_DELTAS: [Square; 8] = [
        Square {
            column: -2,
            row: -1,
        },
        Square { column: -2, row: 1 },
        Square { column: 2, row: -1 },
        Square { column: 2, row: 1 },
        Square { column: 1, row: 2 },
        Square { column: -1, row: 2 },
        Square { column: 1, row: -2 },
        Square {
            column: -1,
            row: -2,
        },
    ];

    // Find all possible moves within the given board.
    fn neighbors(&self, columns: isize, rows: isize) -> Vec<Square> {
        Self::KNIGHT_DELTAS
            .iter()
            .map(|d| self + d)
            .filter(|p| p.column >= 0 && p.column < columns && p.row >= 0 && p.row < rows)
            .collect()
    }
}

impl Add<Square> for Square {
    type Output = Square;

    fn add(self, rhs: Square) -> Self::Output {
        Square {
            column: self.column + rhs.column,
            row: self.row + rhs.row,
        }
    }
}

impl Add<&Square> for &Square {
    type Output = Square;

    fn add(self, rhs: &Square) -> Self::Output {
        Square {
            column: self.column + rhs.column,
            row: self.row + rhs.row,
        }
    }
}

// We can just BFS all the moves.
fn p1((dragon, sheep, _, rows, columns): &InputPart1, moves: usize) -> usize {
    // Maintain the list of work to do and previously seen Squares.
    let mut frontier = VecDeque::new();
    frontier.push_back((*dragon, 0));
    let mut seen = FxHashSet::default();
    seen.insert(*dragon);

    let mut total = 0;
    while let Some((point, steps)) = frontier.pop_front() {
        // If we have a sheep, we can mark it.
        if sheep.contains(&point) {
            total += 1;
        }

        // If we've exhausted our steps, we'll stop this path.
        if steps == moves {
            continue;
        }

        // Add all of our next moves that we haven't been to before.
        for neighbor in point.neighbors(*columns, *rows) {
            if seen.insert(neighbor) {
                frontier.push_back((neighbor, steps + 1));
            }
        }
    }

    total
}

// The goal here is to mark all the places the dragon can go and then move the sheep around and see
// if they'd be at the save place and time as the dragon. My original implementation was a sort of
// start graph but it was too slow for the puzzle input (worked on examples).
fn p2((dragon, sheep, hideouts, rows, columns): &InputPart2, moves: usize) -> usize {
    // Find all the places the dragon can be at a given step.
    let mut frontier = VecDeque::new();
    frontier.push_back((*dragon, 0));
    let mut dragons = FxHashSet::default();
    dragons.insert((*dragon, 0));
    while let Some((point, steps)) = frontier.pop_front() {
        if steps == moves {
            continue;
        }
        for neighbor in point.neighbors(*columns, *rows) {
            if dragons.insert((neighbor, steps + 1)) {
                frontier.push_back((neighbor, steps + 1));
            }
        }
    }

    // Iterate through all the moves the sheep can make over all the moves and see if it would
    // collide with a dragon where there isn't a hideout.
    let mut sheep = sheep.clone();
    let original = sheep.len();
    for step in 0..=moves {
        sheep.retain(|sheep| {
            let p = *sheep + Square::new(0, step);
            hideouts.contains(&p)
                || (!dragons.contains(&(p, step)) && !dragons.contains(&(p, step + 1)))
            // Note we are checking this step and the next step because the sheep could walk
            // into the dragon here.
        });
    }

    original - sheep.len()
}

// Track whose turn it is.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Turn {
    Dragon,
    Sheep,
}

// Track the state of a given node in the graph. These three values represent where we are and if
// we've seen this state before, we should already know the answer.
#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    sheep: Vec<Option<isize>>,
    dragon: Square,
    turn: Turn,
}

impl State {
    fn new(sheep: Vec<Option<isize>>, dragon: Square, turn: Turn) -> Self {
        State {
            sheep: sheep.clone(),
            dragon,
            turn,
        }
    }
}

struct Board {
    hideouts: FxHashSet<Square>,
    columns: isize,
    rows: isize,
    seen: FxHashMap<State, usize>,
}

impl Board {
    fn sheep_turn(&mut self, state: State) -> usize {
        let mut total = 0;
        let mut moved = false;

        for column in 0..self.columns {
            // We only handle a row if we have a sheep there right now.
            if let Some(row) = state.sheep[column as usize] {
                // Is it at the end? If so, the result of this is "0", but we did move, so the
                // dragon shouldn't take a turn after this. Also check if the rest of the column
                // are hideouts. That's also a loss.
                if row == self.rows - 1
                    || (row + 1..self.rows).all(|r| {
                        self.hideouts
                            .contains(&Square::new(column as usize, r as usize))
                    })
                {
                    moved = true;
                    continue;
                }

                // We can't walk into a dragon unless it's on a hideout.
                if row + 1 == state.dragon.row
                    && column == state.dragon.column
                    && !self.hideouts.contains(&state.dragon)
                {
                    continue;
                }

                // If we got here, we can move, so move the column down and continue the DFS.
                moved = true;
                let mut new_sheep = state.sheep.clone();
                new_sheep[column as usize] = Some(row + 1);
                total += self.dfs(State::new(new_sheep, state.dragon, Turn::Dragon));
            }
        }

        // We need to track if a sheep was able to move. If one hasn't moved, then we take a dragon
        // turn because we won't have done it in the loop.
        if !moved {
            return self.dfs(State::new(state.sheep, state.dragon, Turn::Dragon));
        }

        total
    }

    fn dragon_turn(&mut self, state: State) -> usize {
        // Find all the possible dragon moves and figure out DFS recursively. We sum the results to
        // get the total for this state.
        state
            .dragon
            .neighbors(self.columns, self.rows)
            .iter()
            .map(|next| {
                // Remove a sheep if we eat it.
                let mut sheep = state.sheep.to_vec();
                if let Some(row) = sheep[next.column as usize]
                    && next.row == row
                    && !self.hideouts.contains(next)
                {
                    sheep[next.column as usize] = None;
                }
                self.dfs(State::new(sheep, *next, Turn::Sheep))
            })
            .sum()
    }

    fn dfs(&mut self, state: State) -> usize {
        // If all the sheep are eaten, we win. If we've seen this state before, not need to
        // continue.
        if state.sheep.iter().all(|s| s.is_none()) {
            return 1;
        } else if let Some(&v) = self.seen.get(&state) {
            // I got 2_960_763 hits in my input.
            return v;
        }

        // handle the turn.
        let result = match state.turn {
            Turn::Sheep => self.sheep_turn(state.clone()),
            Turn::Dragon => self.dragon_turn(state.clone()),
        };

        // Update our seen with this result and return it.
        self.seen.insert(state, result);
        result
    }
}

// We do a DFS with pruning
fn p3((dragon, sheep, hideouts, rows, columns): &InputPart3) -> usize {
    // Create our board to manage resources.
    let mut board = Board {
        hideouts: hideouts.clone(),
        columns: *columns,
        rows: *rows,
        seen: FxHashMap::default(),
    };

    // Track the sheep as a single vector as there is now only one in each row.
    let sheep = sheep
        .iter()
        .fold(vec![None; board.columns as usize], |mut acc, sheep| {
            acc[sheep.column as usize] = Some(sheep.row);
            acc
        });
    let state = State::new(sheep, *dragon, Turn::Sheep);

    board.dfs(state)
}

fn main() {
    let now = Instant::now();
    let input = parse_input_part1(INPUT_PART1);
    let solution = p1(&input, 4);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part2(INPUT_PART2);
    let solution = p2(&input, 20);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part3(INPUT_PART3);
    let solution = p3(&input);
    println!("p3 {:?} {}", now.elapsed(), solution);
}

const INPUT_PART2: &str = include_str!("inputs/quest10-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest10-3.txt");
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
            "...SSS.......
.S......S.SS.
..S....S...S.
..........SS.
..SSSS...S...
.....SS..S..S
SS....D.S....
S.S..S..S....
....S.......S
.SSS..SS.....
.........S...
.......S....S
SS.....S..S..",
        );
        assert_eq!(p1(&input, 3), 27);
    }

    #[test]
    fn test_p2() {
        let input = parse_input_part2(
            "...SSS##.....
.S#.##..S#SS.
..S.##.S#..S.
.#..#S##..SS.
..SSSS.#.S.#.
.##..SS.#S.#S
SS##.#D.S.#..
S.S..S..S###.
.##.S#.#....S
.SSS.#SS..##.
..#.##...S##.
.#...#.S#...S
SS...#.S.#S..",
        );
        assert_eq!(p2(&input, 3), 27);
    }

    #[test]
    fn test_p3() {
        let input = parse_input_part3("SSS\n..#\n#.#\n#D.");
        assert_eq!(p3(&input), 15);
    }

    #[test]
    fn test_p3_2() {
        let input = parse_input_part3("SSS\n..#\n..#\n.##\n.D#");
        assert_eq!(p3(&input), 8);
    }

    #[test]
    fn test_p3_3() {
        // I had a crazy bug here where I had pasted the example with spaces in-front and behind.
        let input = parse_input_part3("..S..\n.....\n..#..\n.....\n..D..");
        assert_eq!(p3(&input), 44);
    }

    #[test]
    fn test_p3_4() {
        let input = parse_input_part3(".SS.S\n#...#\n...#.\n##..#\n.####\n##D.#");
        assert_eq!(p3(&input), 4406);
    }
}
