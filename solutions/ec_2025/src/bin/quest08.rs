use std::time::Instant;

use itertools::Itertools;

const INPUT_PART1: &str = include_str!("inputs/quest08-1.txt");
type InputPart1<'a> = Vec<(isize, isize)>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .trim()
        .split(',')
        // We do -1 here to make the math simpler?
        .map(|v| v.parse::<isize>().unwrap() - 1)
        .tuple_windows()
        .map(|(l, r)| (l.min(r), r.max(l)))
        .collect()
}

fn p1(strings: &InputPart1, size: isize) -> usize {
    strings
        .iter()
        .filter(|(l, r)| (l - r).abs() == size / 2)
        .count()
}

fn overlap((s1, e1): (isize, isize), (s2, e2): (isize, isize)) -> bool {
    // The strings overlap if one side is between start and end and the other side is
    // between end and start.
    (s2 > s1 && s2 < e1 && (e2 > e1 || e2 < s1))
        || (e2 > s1 && e2 < e1 && (s2 > e1 || s2 < s1))
        || (s2 == s1 && e2 == e1)
        || (s2 == e1 && e2 == s1)
}

fn p2(strings: &InputPart2) -> usize {
    strings
        .iter()
        .scan(Vec::new(), |strings, cur| {
            // Count all the strings that overlap.
            let count = strings.iter().filter(|t| overlap(**t, *cur)).count();

            // Add this string to our list.
            strings.push(*cur);

            // Return the count to be summed with the rest.
            Some(count)
        })
        .sum::<usize>()
}

fn p3(strings: &InputPart3, size: isize) -> usize {
    // Try all possible cuts.
    let mut max = 0;
    for start in 0..=(size / 2) {
        for end in (size / 2)..size {
            let cuts = strings
                .iter()
                .filter(|t| overlap(**t, (start, end)))
                .count();
            max = max.max(cuts);
        }
    }

    max
}

fn main() {
    let now = Instant::now();
    let input = parse_input_part1(INPUT_PART1);
    let solution = p1(&input, 32);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part2(INPUT_PART2);
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part3(INPUT_PART3);
    let solution = p3(&input, 256);
    println!("p3 {:?} {}", now.elapsed(), solution);
}

const INPUT_PART2: &str = include_str!("inputs/quest08-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest08-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = parse_input_part1("1,5,2,6,8,4,1,7,3");
        assert_eq!(p1(&input, 8), 4);
    }

    #[test]
    fn test_p2() {
        let input = parse_input_part2("1,5,2,6,8,4,1,7,3,5,7,8,2");
        assert_eq!(p2(&input), 21);
    }

    #[test]
    fn test_p3() {
        let input = parse_input_part3("1,5,2,6,8,4,1,7,3,6");
        assert_eq!(p3(&input, 8), 7);
        assert_eq!(input, input);
    }
}

