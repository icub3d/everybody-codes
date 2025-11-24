use std::{str::FromStr, time::Instant};

use thiserror::Error;

#[derive(Debug, Copy, Clone, Error)]
enum Error {
    #[error("parsing: {0}")]
    ParseError(&'static str),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn value(&self) -> usize {
        self.x + 100 * self.y
    }

    // Determine which disc it's on.
    fn disc(&self) -> usize {
        let mut cur = *self;
        while cur.y != 1 {
            cur.x += 1;
            cur.y -= 1;
        }
        cur.x
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s
            .split_once(' ')
            .ok_or(Error::ParseError("missing space"))?;

        let (_, x) = l
            .split_once('=')
            .ok_or(Error::ParseError("missing equal"))?;
        let x = x
            .parse::<usize>()
            .map_err(|_| Error::ParseError("parsing x"))?;

        let (_, y) = r
            .split_once('=')
            .ok_or(Error::ParseError("missing equal"))?;
        let y = y
            .parse::<usize>()
            .map_err(|_| Error::ParseError("parsing y"))?;

        Ok(Point { x, y })
    }
}

const INPUT_PART1: &str = include_str!("inputs/quest03-1.txt");
type InputPart1<'a> = Vec<Point>;
fn parse_input_part1(input: &'_ str) -> Result<InputPart1<'_>, Error> {
    input
        .lines()
        .map(Point::from_str)
        .collect::<Result<Vec<_>, Error>>()
}

fn p1(input: &mut InputPart1) -> usize {
    for _ in 0..100 {
        for p in input.iter_mut() {
            if p.y == 1 {
                p.y = p.x;
                p.x = 1;
            } else {
                p.x += 1;
                p.y -= 1;
            }
        }
    }

    input.iter().map(|p| p.value()).sum::<usize>()
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a.abs(), a.signum(), 0)
    } else {
        let (g, s1, t1) = extended_gcd(b, a % b);
        (g, t1, s1 - (a / b) * t1)
    }
}

/// normalize to [0, m)
fn norm(x: i128, m: i128) -> i128 {
    let mut r = x % m;
    if r < 0 {
        r += m;
    }
    r
}

fn crt(pairs: &[(usize, usize)]) -> Option<(usize, usize)> {
    if pairs.is_empty() {
        return Some((0, 1));
    }

    let mut r0 = pairs[0].0 as i128;
    let mut m0 = pairs[0].1 as i128;
    if m0 <= 0 {
        return None;
    }
    r0 = norm(r0, m0);

    for &(ri_u, mi_u) in pairs.iter().skip(1) {
        let mut r1 = ri_u as i128;
        let m1 = mi_u as i128;
        if m1 <= 0 {
            return None;
        }
        r1 = norm(r1, m1);

        let (g, s, _t) = extended_gcd(m0, m1);
        let diff = r1 - r0;
        if diff % g != 0 {
            return None; // inconsistent
        }

        // t ≡ (diff / g) * s  (mod m1/g)
        let m1_div_g = m1 / g;
        let t_val = norm((diff / g) * s, m1_div_g);

        let new_r = r0 + m0 * t_val;
        let lcm = (m0 / g) * m1;

        r0 = norm(new_r, lcm);
        m0 = lcm;
    }

    if r0 < 0 || m0 <= 0 {
        return None;
    }
    Some((r0 as usize, m0 as usize))
}

fn p2(input: &mut InputPart2) -> usize {
    // For each snail, we have a congruence t ≡ y - 1 (mod m)
    // where m is the disc size (x + y - 1).
    // https://cp-algorithms.com/algebra/chinese-remainder-theorem.html
    let pairs = input
        .iter()
        .map(|p| {
            let m = p.disc();
            let r = p.y - 1;
            (r, m)
        })
        .collect::<Vec<_>>();
    crt(&pairs).map(|(r, _)| r).unwrap_or(0)
}

fn p3(input: &mut InputPart3) -> usize {
    p2(input)
}

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let mut input = parse_input_part1(INPUT_PART1)?;
    let solution = p1(&mut input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let mut input = parse_input_part2(INPUT_PART2)?;
    let solution = p2(&mut input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let mut input = parse_input_part3(INPUT_PART3)?;
    let solution = p3(&mut input);
    println!("p3 {:?} {}", now.elapsed(), solution);

    Ok(())
}

const INPUT_PART2: &str = include_str!("inputs/quest03-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> Result<InputPart2<'_>, Error> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest03-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> Result<InputPart3<'_>, Error> {
    parse_input_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() -> anyhow::Result<()> {
        let mut input = parse_input_part1("x=1 y=2\nx=2 y=3\nx=3 y=4\nx=4 y=4")?;
        assert_eq!(p1(&mut input), 1310);
        Ok(())
    }

    #[test]
    fn test_p2() -> anyhow::Result<()> {
        let mut input = parse_input_part1("x=12 y=2\nx=8 y=4\nx=7 y=1\nx=1 y=5\nx=1 y=3")?;
        assert_eq!(p2(&mut input), 14);
        Ok(())
    }

    #[test]
    fn test_p2_big() -> anyhow::Result<()> {
        let mut input = parse_input_part1("x=3 y=1\nx=3 y=9\nx=1 y=5\nx=4 y=10\nx=5 y=3")?;
        assert_eq!(p2(&mut input), 13659);
        Ok(())
    }

    #[test]
    fn test_p3() -> anyhow::Result<()> {
        let mut input = parse_input_part1("x=3 y=1\nx=3 y=9\nx=1 y=5\nx=4 y=10\nx=5 y=3")?;
        assert_eq!(p2(&mut input), 13659);
        Ok(())
    }
}
