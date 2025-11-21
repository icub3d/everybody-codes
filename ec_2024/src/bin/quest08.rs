use std::time::Instant;

const INPUT_PART1: &str = include_str!("inputs/quest08-1.txt");
type InputPart1<'a> = u64;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input.trim().parse::<u64>().unwrap()
}

fn p1(input: &InputPart1) -> u64 {
    let mut cur = 0;
    for layer in (1..).step_by(2) {
        cur += layer;
        if cur > *input {
            return (cur - input) * layer;
        }
    }
    unreachable!()
}

fn p2(input: &InputPart2, acolytes: u64, available: u64) -> u64 {
    let mut cur = 1;
    let mut thickness = 1;
    for width in (3..).step_by(2) {
        thickness = (thickness * input) % acolytes;
        cur += thickness * width;
        if cur > available {
            return (cur - available) * width;
        }
    }
    unreachable!()
}

fn p3(input: &InputPart2, acolytes: u64, available: u64) -> u64 {
    let mut cur = 1;
    let mut thickness = 1;
    let mut thicknesses = vec![1];
    for width in (3..).step_by(2) {
        thickness = (thickness * input) % acolytes + acolytes;
        thicknesses.push(thickness);
        cur += thickness * width;

        let sum = thicknesses.iter().sum::<u64>();
        let mut removed = (sum * input * width) % acolytes;
        for i in 1..width / 2 {
            let sum = thicknesses[i as usize..thicknesses.len()]
                .iter()
                .sum::<u64>();
            let remove = (sum * input * width) % acolytes;
            removed += remove * 2;
        }

        if (cur - removed) > available {
            return cur - removed - available;
        }
    }
    unreachable!()
}

fn main() {
    let now = Instant::now();
    let input = parse_input_part1(INPUT_PART1);
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part2(INPUT_PART2);
    let solution = p2(&input, 1111, 20240000);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part3(INPUT_PART3);
    let solution = p3(&input, 10, 202400000);
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

