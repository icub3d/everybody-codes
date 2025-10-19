use std::time::Instant;

const INPUT1: &str = include_str!("inputs/[QUEST]-1.txt");
type Input1<'a> = Vec<&'a str>;
fn parse_input1(input: &'_ str) -> Input1<'_> {
    // TODO: have you trim today?
    input.trim().lines().collect()
}

fn p1() -> usize {
    let _input = parse_input1(INPUT1);
    0
}

// const INPUT2: &str = include_str!("inputs/[QUEST]-2.txt");
// type Input2<'a> = Vec<&'a str>;
// fn parse_input2(input: &'_ str) -> Input2<'_> {
//     // TODO: have you trim today?
//     input.trim().lines().collect()
// }

fn p2() -> usize {
    // let input = parse_input2(INPUT2);
    0
}

// const INPUT3: &str = include_str!("inputs/[QUEST]-3.txt");
// type Input3<'a> = Vec<&'a str>;
// fn parse_input3(input: &'_ str) -> Input3<'_> {
//     // TODO: have you trim today?
//     input.trim().lines().collect()
// }

fn p3() -> usize {
    // let input = parse_input3(INPUT3);
    0
}

fn main() {
    let now = Instant::now();
    let solution = p1();
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2();
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p3();
    println!("p3 {:?} {}", now.elapsed(), solution);
}
