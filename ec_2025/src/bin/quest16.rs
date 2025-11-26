use std::time::Instant;

use num::Integer;
use rustc_hash::FxHashSet;

const INPUT_PART1: &str = include_str!("inputs/quest16-1.txt");
type InputPart1<'a> = Vec<usize>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn p1(input: &InputPart1) -> usize {
    (1..=90)
        .map(|n| input.iter().filter(|v| n.is_multiple_of(v)).count())
        .sum()
}

fn factors(n: usize) -> impl Iterator<Item = usize> {
    // The if block handles perfect squares.
    (1..=((n as f64).sqrt() as usize))
        .filter(move |i| n.is_multiple_of(*i))
        .flat_map(move |i| if i != n / i { vec![i, n / i] } else { vec![i] })
}

fn spell_required(input: &InputPart2) -> FxHashSet<usize> {
    (1..)
        .zip(input)
        .fold(FxHashSet::default(), |mut seen, (n, &cur)| {
            let used = factors(n).filter(|f| seen.contains(f)).count();
            if cur > used {
                seen.insert(n);
            }
            seen
        })
}

fn p2(input: &InputPart2) -> usize {
    spell_required(input).iter().product()
}

fn p3(input: &InputPart3) -> usize {
    // Determine the spell.
    let spell = spell_required(input);

    // bsearch the solution space.
    const TOTAL: usize = 202_520_252_025_000;
    bsearch(&spell, TOTAL)
}

fn bsearch(spell: &FxHashSet<usize>, total: usize) -> usize {
    // Do a binary search to find the largest value without going over.
    let mut low = 0;
    let mut high = total;
    while low != high {
        // Find the middle of our current chunk.
        let mid = low + (high - low).div_ceil(2);

        // Find the blocks need for mid and see if it's usable.
        let needed = spell.iter().map(|&s| mid / s).sum::<usize>();
        if needed <= total {
            // This is our best so far, but we can try higher numbers.
            low = mid;
        } else {
            // This is too much, go smaller.
            high = mid - 1;
        }
    }
    low
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

const INPUT_PART2: &str = include_str!("inputs/quest16-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest16-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = parse_input_part1("1,2,3,5,9");
        assert_eq!(p1(&input), 193);
    }

    #[test]
    fn test_p2() {
        let input = parse_input_part2("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2");
        assert_eq!(p2(&input), 270);
    }

    #[test]
    fn test_p3() {
        let input = parse_input_part3("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2");
        assert_eq!(p3(&input), 94439495762954);
    }
}
