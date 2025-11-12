use std::time::Instant;

const INPUT_PART1: &str = include_str!("inputs/[QUEST]-1.txt");
type InputPart1<'a> = Vec<&'a str>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    // TODO: have you trim today?
    input.trim().lines().collect()
}

fn p1(_input: &InputPart1) -> usize {
    0
}

fn p2(_input: &InputPart2) -> usize {
    0
}

fn p3(_input: &InputPart3) -> usize {
    0
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

const INPUT_PART2: &str = include_str!("inputs/[QUEST]-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/[QUEST]-3.txt");
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
