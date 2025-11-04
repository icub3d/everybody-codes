use std::time::Instant;

const INPUT_PART1: &str = include_str!("inputs/quest03-1.txt");
type InputPart1<'a> = Vec<Vec<usize>>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => 0,
                    _ => 1, // #
                })
                .collect()
        })
        .collect()
}

fn p1(input: &InputPart1) -> usize {
    let mut input = input.clone();
    for n in 2.. {
        let mut diggable: Vec<(usize, usize)> = Vec::new();

        for y in 0..input.len() {
            for (x, v) in input[y].iter().enumerate() {
                if *v != n - 1
                    || input[y - 1][x] != n - 1
                    || input[y + 1][x] != n - 1
                    || input[y][x - 1] != n - 1
                    || input[y][x + 1] != n - 1
                {
                    continue;
                }

                diggable.push((x, y));
            }
        }

        if diggable.is_empty() {
            break;
        }

        diggable.iter().for_each(|&(x, y)| input[y][x] = n);
    }

    input.iter().map(|r| r.iter().sum::<usize>()).sum()
}

const INPUT_PART2: &str = include_str!("inputs/quest03-2.txt");
type InputPart2<'a> = Vec<Vec<usize>>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => 0,
                    _ => 1, // #
                })
                .collect()
        })
        .collect()
}

fn p2(input: &InputPart2) -> usize {
    let mut input = input.clone();
    for n in 2.. {
        let mut diggable: Vec<(usize, usize)> = Vec::new();

        for y in 0..input.len() {
            for (x, v) in input[y].iter().enumerate() {
                if *v != n - 1
                    || input[y - 1][x] != n - 1
                    || input[y + 1][x] != n - 1
                    || input[y][x - 1] != n - 1
                    || input[y][x + 1] != n - 1
                {
                    continue;
                }
                diggable.push((x, y));
            }
        }

        if diggable.is_empty() {
            break;
        }

        diggable.iter().for_each(|&(x, y)| input[y][x] = n);
    }

    input.iter().map(|r| r.iter().sum::<usize>()).sum()
}

const INPUT_PART3: &str = include_str!("inputs/quest03-3.txt");
type InputPart3<'a> = Vec<Vec<usize>>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => 0,
                    _ => 1, // #
                })
                .collect()
        })
        .collect()
}

fn p3(input: &InputPart3) -> usize {
    let mut input = input.clone();
    for n in 2.. {
        let mut diggable: Vec<(usize, usize)> = Vec::new();

        for y in 0..input.len() {
            for (x, v) in input[y].iter().enumerate() {
                // Check for n-1 and not on a border.
                if *v != n - 1
                    || x == 0
                    || y == 0
                    || x == input[y].len() - 1
                    || y == input.len() - 1
                {
                    continue;
                }

                // Check all surrounding
                if input[y - 1][x] != n - 1
                    || input[y + 1][x] != n - 1
                    || input[y][x - 1] != n - 1
                    || input[y][x + 1] != n - 1
                    || input[y - 1][x - 1] != n - 1
                    || input[y + 1][x + 1] != n - 1
                    || input[y - 1][x + 1] != n - 1
                    || input[y + 1][x - 1] != n - 1
                {
                    continue;
                }

                diggable.push((x, y));
            }
        }

        if diggable.is_empty() {
            break;
        }

        diggable.iter().for_each(|&(x, y)| input[y][x] = n);
    }

    input.iter().map(|r| r.iter().sum::<usize>()).sum()
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

// ..........
// ..###.##..
// ...####...
// ..######..
// ..######..
// ...####...
// ..........
