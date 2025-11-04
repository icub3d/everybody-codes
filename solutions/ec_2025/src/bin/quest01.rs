use std::time::Instant;

const INPUT_PART1: &str = include_str!("inputs/quest01-1.txt");
type InputPart1<'a> = (Vec<&'a str>, Vec<i32>);
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    let (names, moves) = input.split_once("\n\n").unwrap();
    let names = names.split(',').collect();
    let moves = moves
        .split(',')
        .map(|p| {
            p[1..].parse::<i32>().unwrap()
                * match p.chars().next().unwrap() {
                    'R' => 1,
                    _ => -1,
                }
        })
        .collect();
    (names, moves)
}

fn p1<'a>((names, moves): &'a InputPart1) -> &'a str {
    let max = (names.len() - 1) as i32;
    // let pos = moves
    //     .iter()
    //     .fold(0i32, |acc, delta| (acc + delta).clamp(0, max));
    let pos = moves
        .iter()
        .fold(0i32, |acc, delta| (acc + delta).min(max).max(0));
    names[pos as usize]
}

fn p2<'a>((names, moves): &'a InputPart2) -> &'a str {
    // let pos = moves.iter().sum::<i32>().rem_euclid(names.len() as i32);
    let pos = moves.iter().fold(0i32, |acc, delta| {
        (acc + delta).rem_euclid(names.len() as i32)
    });
    names[pos as usize]
}

fn p3<'a>((names, moves): &'a mut InputPart3) -> &'a str {
    moves.iter().for_each(|delta| {
        let pos = delta.rem_euclid(names.len() as i32);
        names.swap(0, pos as usize);
    });

    names[0]
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
    let mut input = parse_input_part3(INPUT_PART3);
    let solution = p3(&mut input);
    println!("p3 {:?} {}", now.elapsed(), solution);
}

const INPUT_PART2: &str = include_str!("inputs/quest01-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest01-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

