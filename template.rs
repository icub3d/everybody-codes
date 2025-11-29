use std::time::Instant;

const INPUT_PART1: &str = include_str!("inputs/[NAME]-1.txt");
const INPUT_PART2: &str = include_str!("inputs/[NAME]-2.txt");
const INPUT_PART3: &str = include_str!("inputs/[NAME]-3.txt");

fn parse(input: &str) -> Vec<Vec<char>> {
    // TODO: have you trim today?
    input.trim().lines().map(|l| l.chars().collect()).collect()
}

fn p1(input: &str) -> usize {
    let input = parse(input);
    input.len()
}

fn p2(input: &str) -> usize {
    let input = parse(input);
    input.len()
}

fn p3(input: &str) -> usize {
    let input = parse(input);
    input.len()
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
        let input = "123\n456\n789\n";
        assert_eq!(p1(input), 3);
    }

    #[test]
    fn test_p2() {
        let input = "123\n456\n789\n";
        assert_eq!(p2(input), 3);
    }

    #[test]
    fn test_p3() {
        let input = "123\n456\n789\n";
        assert_eq!(p3(input), 3);
    }
}
