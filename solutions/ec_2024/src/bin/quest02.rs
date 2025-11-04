use std::time::Instant;

use rustc_hash::FxHashSet;

const INPUT_PART1: &str = include_str!("inputs/quest02-1.txt");
type InputPart1<'a> = (Vec<&'a str>, Vec<&'a str>);
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    let (runes, inscription) = input.split_once("\n\n").unwrap();
    (
        runes.trim_start_matches("WORDS:").split(',').collect(),
        inscription.split_whitespace().collect(),
    )
}

fn p1((runes, inscription): &InputPart1) -> usize {
    inscription
        .iter()
        .map(|word| {
            runes
                .iter()
                .map(|rune| match word.contains(rune) {
                    true => 1,
                    false => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

const INPUT_PART2: &str = include_str!("inputs/quest02-2.txt");
type InputPart2<'a> = (Vec<&'a str>, &'a str);
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    let (runes, inscription) = input.split_once("\n\n").unwrap();
    (
        runes.trim_start_matches("WORDS:").split(',').collect(),
        inscription.trim(),
    )
}

fn p2((runes, inscription): &InputPart2) -> usize {
    let mut seen: FxHashSet<usize> = FxHashSet::default();

    for rune in runes {
        // Check for matches in original
        for (index, _) in inscription.match_indices(rune) {
            (index..index + rune.len()).for_each(|i| {
                seen.insert(i);
            });
        }
        // Check in reverse but make sure to add the position
        // in the original string.
        let rev = inscription.chars().rev().collect::<String>();
        for (index, _) in rev.match_indices(rune) {
            (index..index + rune.len()).for_each(|i| {
                seen.insert(inscription.len() - i - 1);
            });
        }
    }

    seen.len()
}

const INPUT_PART3: &str = include_str!("inputs/quest02-3.txt");
type InputPart3<'a> = (Vec<Vec<char>>, Vec<Vec<char>>);
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    let (runes, inscription) = input.split_once("\n\n").unwrap();
    (
        runes
            .trim_start_matches("WORDS:")
            .split(',')
            .map(|w| w.chars().collect())
            .collect(),
        inscription.lines().map(|l| l.chars().collect()).collect(),
    )
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_rune(
    seen: &mut FxHashSet<(usize, usize)>,
    rune: &[char],
    inscription: &[Vec<char>],
    x: isize,
    y: isize,
    direction: Direction,
) {
    let xlen = inscription[0].len() as isize;
    let ylen = inscription.len() as isize;
    // Check that rune can be found here.
    for (i, c) in rune.iter().enumerate() {
        let i = i as isize;
        let (x, y) = match direction {
            Direction::Up => {
                if y - i < 0 {
                    return;
                }
                (x, y - i)
            }
            Direction::Down => {
                if y + i >= ylen {
                    return;
                }
                (x, y + i)
            }
            Direction::Left => ((x - i).rem_euclid(xlen), y),
            Direction::Right => ((x + i).rem_euclid(xlen), y),
        };
        if inscription[y as usize][x as usize] != *c {
            return;
        }
    }
    // At this point, we found some, let's do the update.
    for (i, _) in rune.iter().enumerate() {
        let i = i as isize;
        let (x, y) = match direction {
            Direction::Up => (x, y - i),
            Direction::Down => (x, y + i),
            Direction::Left => ((x - i).rem_euclid(xlen), y),
            Direction::Right => ((x + i).rem_euclid(xlen), y),
        };
        seen.insert((x as usize, y as usize));
    }
}

fn p3((runes, inscription): &InputPart3) -> usize {
    let mut seen: FxHashSet<(usize, usize)> = FxHashSet::default();

    for rune in runes {
        for y in 0..inscription.len() {
            for x in 0..inscription[y].len() {
                let x = x as isize;
                let y = y as isize;
                find_rune(&mut seen, rune, inscription, x, y, Direction::Up);
                find_rune(&mut seen, rune, inscription, x, y, Direction::Down);
                find_rune(&mut seen, rune, inscription, x, y, Direction::Left);
                find_rune(&mut seen, rune, inscription, x, y, Direction::Right);
            }
        }
    }

    seen.len()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let runes = vec!["THE", "OWE", "MES", "ROD", "HER"];
        let inscription = "AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE"
            .split_ascii_whitespace()
            .collect();
        assert_eq!(p1(&(runes, inscription)), 4);
    }

    #[test]
    fn test_p3() {
        let runes = "THE,OWE,MES,ROD,RODEO"
            .split(',')
            .map(|w| w.chars().collect())
            .collect();
        let inscription = "HELWORLT\nENIGWDXL\nTRODEOAL"
            .lines()
            .map(|l| l.chars().collect())
            .collect();
        assert_eq!(p3(&(runes, inscription)), 10);
    }
}
