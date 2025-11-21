use std::time::Instant;

const INPUT_PART1: &str = include_str!("inputs/quest11-1.txt");
type InputPart1<'a> = Vec<usize>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

// For p1, simply run the simulation as described.
fn p1(input: &InputPart1) -> usize {
    let mut input = input.clone();

    // Shift right phase.
    let mut round = 0;
    while round < 10 {
        let mut moved = false;
        for i in 0..input.len() - 1 {
            if input[i] > input[i + 1] {
                moved = true;
                input[i] -= 1;
                input[i + 1] += 1;
            }
        }

        if !moved {
            break;
        }
        round += 1;
    }

    // Shift left phase (up to 10 rounds).
    while round < 10 {
        for i in 0..input.len() - 1 {
            if input[i] < input[i + 1] {
                input[i] += 1;
                input[i + 1] -= 1;
            }
        }

        round += 1;
    }

    (1..).zip(input.iter()).map(|(i, n)| i * n).sum::<usize>()
}

// For p2, do the same but run until it's equalized.
fn p2(input: &InputPart2) -> usize {
    let mut input = input.clone();

    // Shift right phase.
    let mut round = 0;
    loop {
        let mut moved = false;
        for i in 0..input.len() - 1 {
            if input[i] > input[i + 1] {
                moved = true;
                input[i] -= 1;
                input[i + 1] += 1;
            }
        }

        if !moved {
            break;
        }
        round += 1;
    }

    // Shift left phase (we can calculate?)
    let mean = input.iter().sum::<usize>() / input.len();
    round
        + input
            .iter()
            .filter(|&&i| i < mean)
            .map(|i| mean - i)
            .sum::<usize>()
}

// p3 wasn't gonna finish (my input produced 133_914_234_649_730). We have to find some other solution. The input is sorted (unlike examples). We can ignore phase 1. LOL. We just have to figure out how many rounds will it take to fill in the gaps. Get either of the values below the mean or above the mean and sum them (should be same).
fn p3(input: &InputPart3) -> usize {
    let mean = input.iter().sum::<usize>() / input.len();
    input.iter().filter(|&&i| i < mean).map(|i| mean - i).sum()
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

const INPUT_PART2: &str = include_str!("inputs/quest11-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest11-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "9\n1\n1\n4\n9\n6";
        let input = parse_input_part1(input);
        assert_eq!(p1(&input), 109);
    }

    #[test]
    fn test_p2() {
        let input = "9\n1\n1\n4\n9\n6";
        let input = parse_input_part2(input);
        assert_eq!(p2(&input), 11);
    }

    #[test]
    fn test_p3() {
        let input = "3\n4\n4\n4\n7\n8";
        let input = parse_input_part3(input);
        assert_eq!(p3(&input), 5);
    }
}
