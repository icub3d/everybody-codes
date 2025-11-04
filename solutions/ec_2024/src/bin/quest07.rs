use rayon::prelude::*;
use rustc_hash::FxHashSet;
use std::{
    iter::{once, repeat_n},
    time::Instant,
};

const INPUT_PART1: &str = include_str!("inputs/quest07-1.txt");
type InputPart1<'a> = Vec<(&'a str, Vec<i64>)>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .lines()
        .map(|l| {
            let (k, vv) = l.split_once(':').unwrap();
            (
                k,
                vv.split(',')
                    .map(|v| match v {
                        "+" => 1,
                        "=" => 0,
                        "-" => -1,
                        _ => panic!("sad {}", v),
                    })
                    .collect(),
            )
        })
        .collect()
}

fn p1(input: &InputPart1) -> String {
    let mut results = input
        .iter()
        .map(|(k, vv)| {
            (
                k,
                (0..10)
                    .scan(10, |acc, i| {
                        *acc = 0.max(*acc + vv[i % vv.len()]);
                        Some(*acc)
                    })
                    .sum::<i64>(),
            )
        })
        .collect::<Vec<_>>();
    results.sort_by(|a, b| b.1.cmp(&a.1));
    results.iter().map(|(k, _)| **k).collect::<String>()
}

const INPUT_PART2: &str = include_str!("inputs/quest07-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

fn parse_track(track_str: &str) -> Vec<char> {
    let grid: Vec<Vec<char>> = track_str.lines().map(|l| l.chars().collect()).collect();

    // Find 'S' position
    let mut start_pos = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_pos = (x, y);
                break;
            }
        }
    }

    let mut result = vec!['S'];
    let mut pos = start_pos;
    let mut prev_pos = None;

    // Directions: right, down, left, up
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    loop {
        let mut found_next = false;

        for (dx, dy) in &directions {
            let new_x = pos.0 as i32 + dx;
            let new_y = pos.1 as i32 + dy;

            if new_x < 0 || new_y < 0 {
                continue;
            }

            let new_pos = (new_x as usize, new_y as usize);

            // Skip if it's where we came from
            if Some(new_pos) == prev_pos {
                continue;
            }

            // Check bounds
            if new_y as usize >= grid.len() || new_x as usize >= grid[new_y as usize].len() {
                continue;
            }

            let ch = grid[new_y as usize][new_x as usize];

            // Check if it's a valid track character
            if ch == '+' || ch == '-' || ch == '=' || ch == 'S' {
                if ch == 'S' {
                    // We've completed the loop
                    return result;
                }

                result.push(ch);
                prev_pos = Some(pos);
                pos = new_pos;
                found_next = true;
                break;
            }
        }

        if !found_next {
            break;
        }
    }

    result
}

const TRACK_P2: &str = "S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
-                                                                     -
=                                                                     =
+                                                                     +
=                                                                     +
+                                                                     =
=                                                                     =
-                                                                     -
--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-";

fn p2(input: &InputPart2) -> String {
    let track = parse_track(TRACK_P2);
    let mut results = input
        .iter()
        .map(|(k, vv)| {
            (
                k,
                std::iter::repeat_n(track.iter(), 10)
                    .flatten()
                    .skip(1)
                    .chain(once(&'S'))
                    .enumerate()
                    .scan(10, |acc, (i, t)| {
                        let delta = match t {
                            '-' => -1,
                            '+' => 1,
                            _ => vv[i % vv.len()],
                        };
                        *acc = 0.max(*acc + delta);
                        Some(*acc)
                    })
                    .sum::<i64>(),
            )
        })
        .collect::<Vec<_>>();
    results.sort_by(|a, b| b.1.cmp(&a.1));
    results.iter().map(|(k, _)| **k).collect::<String>()
}

const TRACK_P3: &str = "S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-";

const INPUT_PART3: &str = include_str!("inputs/quest07-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

fn build_steps(track: &[char], laps: usize) -> Vec<i64> {
    repeat_n(track.iter(), laps)
        .flatten()
        .skip(1)
        .chain(once(&'S'))
        .map(|ch| match ch {
            '+' => 1,
            '-' => -1,
            _ => 0,
        })
        .collect()
}

fn generate_plans(idx: usize, plan: &mut [i64; 11], out: &mut Vec<[i64; 11]>) {
    if idx == plan.len() {
        out.push(*plan);
        return;
    }

    let mut used = FxHashSet::default();
    for i in idx..plan.len() {
        if !used.insert(plan[i]) {
            continue;
        }

        plan.swap(idx, i);
        generate_plans(idx + 1, plan, out);
        plan.swap(idx, i);
    }
}

fn score_plan(plan: &[i64], steps: &[i64]) -> i64 {
    let plan_len = plan.len();
    let mut pidx = 0usize;
    let (_, total) = steps
        .iter()
        .fold((10i64, 0i64), |(mut energy, mut tot), &step| {
            let delta = if step == 0 { plan[pidx] } else { step };
            energy += delta;
            if energy < 0 {
                energy = 0;
            }
            tot += energy;
            pidx += 1;
            if pidx == plan_len {
                pidx = 0;
            }
            (energy, tot)
        });

    total
}

fn p3(input: &InputPart3) -> usize {
    let track = parse_track(TRACK_P3);
    let steps = build_steps(&track, 2024);

    // There is only one rival plan in the input.
    let rival_plan: Vec<i64> = input[0].1.to_vec();
    let rival_score = score_plan(&rival_plan, &steps);

    // Generate all plans.
    let mut plans: Vec<[i64; 11]> = Vec::with_capacity(27_720);
    let mut initial_plan = [1, 1, 1, 1, 1, 0, 0, 0, -1, -1, -1];
    generate_plans(0, &mut initial_plan, &mut plans);

    // Score all plans in parallel and count those that beat the rival. CPU go brrr
    plans
        .par_iter()
        .filter(|plan| score_plan(plan.as_slice(), &steps) > rival_score)
        .count()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+";
        let input = parse_input_part1(input);
        assert_eq!(p1(&input), "BDCA");
    }
}
