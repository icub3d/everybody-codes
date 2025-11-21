use std::time::Instant;

const INPUT_PART1: &str = include_str!("inputs/quest13-1.txt");
type InputPart1<'a> = Vec<usize>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    // We just get the lines as usize.
    input
        .trim()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

fn p1(input: &InputPart1) -> usize {
    // We can get the left and right sides with iterators.
    let left = input.iter().skip(1).step_by(2).rev().copied(); // Odds, reversed
    let middle = std::iter::once(1);
    let right = input.iter().step_by(2).copied(); // Evens

    // Put them together.
    let wheel = middle.chain(right).chain(left).collect::<Vec<_>>();

    wheel[2025 % wheel.len()]
}

const INPUT_PART2: &str = include_str!("inputs/quest13-2.txt");
type InputPart2<'a> = Vec<(usize, usize)>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    // For p2 and p3 we want to get the ranges instead.
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_once('-')
                .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
                .unwrap()
        })
        .collect()
}

fn make_wheel(input: &InputPart2) -> Vec<(usize, usize, bool)> {
    // We do the same as p1 but the left side will have the numbers reversed so we track if we should be counting backwards.
    let left = input
        .iter()
        .skip(1)
        .step_by(2)
        .rev()
        .map(|(l, r)| (*l, *r, true));
    let middle = std::iter::once((1, 1, false));
    let right = input.iter().step_by(2).map(|(l, r)| (*l, *r, false));
    middle.chain(right).chain(left).collect::<Vec<_>>()
}

fn spin_wheel(wheel: &[(usize, usize, bool)], mut ticks: usize) -> usize {
    // Optimize the iteration by removing full wheel spins from the ticks.
    ticks %= wheel.iter().map(|(l, r, _)| r - l + 1).sum::<usize>();
    for (l, r, backwards) in wheel {
        let diff = r - l;
        if ticks <= diff {
            return match backwards {
                true => r - ticks,
                false => l + ticks,
            };
        }
        ticks -= diff + 1; // off-by-one, LOL.
    }
    unreachable!()
}

fn p2(input: &InputPart2) -> usize {
    spin_wheel(&make_wheel(input), 20252025)
}

fn p3(input: &InputPart3) -> usize {
    spin_wheel(&make_wheel(input), 202520252025)
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

const INPUT_PART3: &str = include_str!("inputs/quest13-3.txt");
type InputPart3<'a> = InputPart2<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = parse_input_part1("72\n58\n47\n61\n67");
        assert_eq!(p1(&input), 67);
    }

    #[test]
    fn test_p2() {
        let input = parse_input_part2("10-15\n12-13\n20-21\n19-23\n30-37");
        assert_eq!(p2(&input), 30);
    }

    #[test]
    fn test_p3() {
        let input = parse_input_part3(INPUT_PART3);
        assert_eq!(input, input);
    }
}
