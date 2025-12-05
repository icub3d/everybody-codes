use std::time::Instant;

use rustc_hash::FxHashMap;

const INPUT_PART1: &str = include_str!("inputs/quest01-1.txt");
const INPUT_PART2: &str = include_str!("inputs/quest01-2.txt");
const INPUT_PART3: &str = include_str!("inputs/quest01-3.txt");

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let grid = grid.lines().map(|l| l.chars().collect()).collect();
    let moves = moves.lines().map(|l| l.chars().collect()).collect();
    (grid, moves)
}

// Calculate score for a given start position and it's moves.
fn score(grid: &[Vec<char>], moves: &[char], start: usize) -> usize {
    let mut col = start;
    let mut row = 0;
    for dir in moves.iter() {
        // if we've reached the end of the grid, we can return the score.
        if row == grid.len() {
            break;
        }
        // Move to the new column taking care of boundaries.
        col = match dir {
            'R' => match col == grid[0].len() - 1 {
                true => col - 1,
                false => col + 1,
            },
            'L' => match col == 0 {
                true => col + 1,
                false => col - 1,
            },
            _ => unreachable!(),
        };

        // Drop to the next bumper (or end).
        while row < grid.len() && grid[row][col] != '*' {
            row += 1;
        }
    }
    ((col / 2 + 1) * 2).saturating_sub(start / 2 + 1)
}

fn p1(input: &str) -> usize {
    let (grid, moves) = parse(input);

    let starts = grid[0]
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '*')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    // We just calculate the score for the start/move pair.
    starts
        .iter()
        .zip(moves.iter())
        .map(|(start, moves)| score(&grid, moves, *start))
        .sum()
}

fn p2(input: &str) -> usize {
    let (grid, moves) = parse(input);

    let starts = grid[0]
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '*')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    // In this case, we want to find the highest score for each start.
    moves
        .iter()
        .map(|moves| {
            starts
                .iter()
                .map(|start| score(&grid, moves, *start))
                .max()
                .unwrap()
        })
        .sum()
}

fn p3(input: &str) -> String {
    let (grid, moves) = parse(input);

    let starts = grid[0]
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '*')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let scores = moves
        .iter()
        .map(|moves| {
            starts
                .iter()
                .map(|start| score(&grid, moves, *start))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let m = dp(
        &scores,
        0,
        0,
        &mut FxHashMap::default(),
        usize::MAX,
        std::cmp::min,
    );

    let n = dp(
        &scores,
        0,
        0,
        &mut FxHashMap::default(),
        usize::MIN,
        std::cmp::max,
    );

    format!("{m} {n}")
}

// P3 is an assignment problem. We can use dynamic programming to solve. TODO describe state, memo,
// recurrence relation. The gist is that if we have seen the same 3 open slots and the same 3 open
// tokens, we don't need to calculate it again. Instead, we can just reuse it. So we sort of "try
// all" but there is a lot of overlap in the state graph.
fn dp<F>(
    scores: &[Vec<usize>],
    cur: usize,
    used: usize,
    memo: &mut FxHashMap<(usize, usize), usize>,
    initial: usize,
    cmp: F,
) -> usize
where
    F: Fn(usize, usize) -> usize + Copy,
{
    if cur == scores.len() {
        return 0;
    }
    if let Some(&val) = memo.get(&(cur, used)) {
        return val;
    }

    let mut m = initial;
    let mut available = !used & ((1 << scores[cur].len()) - 1);
    while available > 0 {
        let slot = available.trailing_zeros() as usize;

        let res = scores[cur][slot] + dp(scores, cur + 1, used | (1 << slot), memo, initial, cmp);
        m = cmp(m, res);

        available &= available - 1; // Clear the LSB
    }

    memo.insert((cur, used), m);
    m
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
        let input = "*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.\n*.*.*...*.*...*..\n.*.*.*.*.*...*.*.\n*.*.....*...*.*.*\n.*.*.*.*.*.*.*.*.\n*...*...*.*.*.*.*\n.*.*.*.*.*.*.*.*.\n*.*.*...*.*.*.*.*\n.*...*...*.*.*.*.\n*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.\n\nRRRLRLRRRRRL\nLLLLRLRRRRRR\nRLLLLLRLRLRL\nLRLLLRRRLRLR\nLLRLLRLLLRRL\nLRLRLLLRRRRL\nLRLLLLLLRLLL\nRRLLLRLLRLRR\nRLLLLLRLLLRL";
        assert_eq!(p1(input), 26);
    }

    #[test]
    fn test_p2() {
        let input = "*.*.*.*.*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.*.*.*.*.\n..*.*.*.*...*.*...*.*.*..\n.*...*.*.*.*.*.*.....*.*.\n*.*...*.*.*.*.*.*...*.*.*\n.*.*.*.*.*.*.*.*.......*.\n*.*.*.*.*.*.*.*.*.*...*..\n.*.*.*.*.*.*.*.*.....*.*.\n*.*...*.*.*.*.*.*.*.*....\n.*.*.*.*.*.*.*.*.*.*.*.*.\n*.*.*.*.*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.*...*.*.\n*.*.*.*.*.*.*.*.*...*.*.*\n.*.*.*.*.*.*.*.*.....*.*.\n*.*.*.*.*.*.*.*...*...*.*\n.*.*.*.*.*.*.*.*.*.*.*.*.\n*.*.*...*.*.*.*.*.*.*.*.*\n.*...*.*.*.*...*.*.*...*.\n*.*.*.*.*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.*.*.*.*.\n\nRRRLLRRRLLRLRRLLLRLR\nRRRRRRRRRRLRRRRRLLRR\nLLLLLLLLRLRRLLRRLRLL\nRRRLLRRRLLRLLRLLLRRL\nRLRLLLRRLRRRLRRLRRRL\nLLLLLLLLRLLRRLLRLLLL\nLRLLRRLRLLLLLLLRLRRL\nLRLLRRLLLRRRRRLRRLRR\nLRLLRRLRLLRLRRLLLRLL\nRLLRRRRLRLRLRLRLLRRL";
        assert_eq!(p2(input), 115);
    }

    #[test]
    fn test_p3() {
        let input = "*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.\n*.*.*...*.*...*..\n.*.*.*.*.*...*.*.\n*.*.....*...*.*.*\n.*.*.*.*.*.*.*.*.\n*...*...*.*.*.*.*\n.*.*.*.*.*.*.*.*.\n*.*.*...*.*.*.*.*\n.*...*...*.*.*.*.\n*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.\n\nRRRLRLRRRRRL\nLLLLRLRRRRRR\nRLLLLLRLRLRL\nLRLLLRRRLRLR\nLLRLLRLLLRRL\nLRLRLLLRRRRL";
        assert_eq!(p3(input), "13 43");
    }

    #[test]
    fn test_p3_bigger() {
        let input = "*.*.*.*.*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.*.*.*.*.\n..*.*.*.*...*.*...*.*.*..\n.*...*.*.*.*.*.*.....*.*.\n*.*...*.*.*.*.*.*...*.*.*\n.*.*.*.*.*.*.*.*.......*.\n*.*.*.*.*.*.*.*.*.*...*..\n.*.*.*.*.*.*.*.*.....*.*.\n*.*...*.*.*.*.*.*.*.*....\n.*.*.*.*.*.*.*.*.*.*.*.*.\n*.*.*.*.*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.*...*.*.\n*.*.*.*.*.*.*.*.*...*.*.*\n.*.*.*.*.*.*.*.*.....*.*.\n*.*.*.*.*.*.*.*...*...*.*\n.*.*.*.*.*.*.*.*.*.*.*.*.\n*.*.*...*.*.*.*.*.*.*.*.*\n.*...*.*.*.*...*.*.*...*.\n*.*.*.*.*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.*.*.*.*.\n\nRRRLLRRRLLRLRRLLLRLR\nRRRRRRRRRRLRRRRRLLRR\nLLLLLLLLRLRRLLRRLRLL\nRRRLLRRRLLRLLRLLLRRL\nRLRLLLRRLRRRLRRLRRRL\nLLLLLLLLRLLRRLLRLLLL";
        assert_eq!(p3(input), "25 66");
    }

    #[test]
    fn test_p3_biggest() {
        let input = "*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.\n..*.*.*.*.*.*.........*.*.*.*.....*.*.*\n.*.*...*.*.*.*.*.*.*.*.*.*.*...*.*.*.*.\n*.*.*.*...*.*.*.*.*.....*.*.*.*...*.*..\n.*...*.*...*.*.*.*.*.*.*.....*.*.*.*.*.\n*.*.*.*.*.....*.*.*.*.*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*...*.*.*.*.....*.*.*.*...*.\n*.*...*.*.*.*.*.*.*.*...*.*.*...*.*.*.*\n.*...*.*.*.*.*.*.*.*...*.*.*.*.*.*.*.*.\n*.*.*.*.*.*...*.....*.*...*...*.*.*.*.*\n.*...*.*.*.*.*...*.*.*.*.*...*.*...*.*.\n*.*.*.*.*...*.*.*.*.*.*.*.*...*.*.*.*.*\n.*.*.*.*.*.*.*.*...*.*.*.*.*.*.*.*.*.*.\n....*.*.*.*...*.*.*.*.*.*.*...*.*.*...*\n.*.*.*...*.*.*.*.*...*.*.*.*.*.*.*.*...\n*.*.*.*.*.*.*.....*...*...*.*.*.*.*.*.*\n.*.*...*.....*.*.*.*.*.*.*...*.*.*.*.*.\n*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*\n.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.\n\nRRRRLLRRLLLLLLLRLLRL\nRRRRRRRLRRLRRLRRRLRR\nRRRLLRRRRRLRRRRRLRRR\nLLLLRRLLRRLLLLLRRLLL\nLRRRRLRRLRLLRLLRRLRR\nRRRRRRRRLRRRRLLRRRLR";
        assert_eq!(p3(input), "39 122");
    }
}
