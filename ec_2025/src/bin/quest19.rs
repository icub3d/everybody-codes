use std::{ops::RangeInclusive, time::Instant};

use itertools::Itertools;
use num::Integer;

const INPUT_PART1: &str = include_str!("inputs/quest19-1.txt");
const INPUT_PART2: &str = include_str!("inputs/quest19-2.txt");
const INPUT_PART3: &str = include_str!("inputs/quest19-3.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Opening {
    dist: isize,
    start: isize,
    size: isize,
}

impl From<&str> for Opening {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',').map(|p| p.parse::<isize>().unwrap());
        Opening {
            dist: parts.next().unwrap(),
            start: parts.next().unwrap(),
            size: parts.next().unwrap(),
        }
    }
}

fn parse(input: &str) -> Vec<Opening> {
    input.trim().lines().map(Opening::from).collect()
}

#[derive(Debug, Clone)]
struct Wall {
    dist: isize,
    gaps: Vec<Range>,
}

impl Wall {
    fn new(dist: isize, gap: Range) -> Self {
        Wall {
            dist,
            gaps: vec![gap],
        }
    }

    // Create all the Walls with the openings merged (assumes input is sorted (it is)).
    fn layout(openings: Vec<Opening>) -> Vec<Self> {
        openings
            .into_iter()
            .fold(Vec::new(), |mut acc: Vec<Wall>, opening| {
                let gap = Range::new(opening.start, opening.start + opening.size - 1);
                // If the new gap is in our last wall, add it, otherwise make a new wall.
                match acc.last_mut() {
                    Some(wall) if wall.dist == opening.dist => {
                        wall.gaps.push(gap);
                    }
                    _ => {
                        acc.push(Wall::new(opening.dist, gap));
                    }
                }
                acc
            })
    }
}

// Describes a range (TODO could use stdlib Range?) that's an opening in a wall or a reachable range from one wall to another.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn new(start: isize, end: isize) -> Self {
        Self { start, end }
    }

    fn is_empty(&self) -> bool {
        self.start > self.end
    }

    // Get the overlap of `self` with `other`.
    fn intersection(&self, other: &Self) -> Self {
        Self::new(self.start.max(other.start), self.end.min(other.end))
    }

    // Expand `self` on either end by the given `amount`.
    fn expand(&self, amount: isize) -> Self {
        Self::new(self.start - amount, self.end + amount)
    }

    // Create an iterator over the range of this.
    fn iter(&self) -> RangeInclusive<isize> {
        self.start..=self.end
    }

    // Determine which nodes are reachable from `self`'s range to the `opening` given the `cur` wall x position and the `next` wall x position.
    fn reachable(&self, opening: &Range, cur: isize, next: isize) -> Option<Range> {
        let dist = next - cur;

        // Determine potential range and intersect with the opening.
        let intersect_range = self.expand(dist).intersection(opening);

        // Find the minimum valid height.
        let min = intersect_range
            .iter()
            .filter(|y| (next - y).is_multiple_of(&2))
            .map(|y| (y, self.intersection(&Range::new(y - dist, y + dist))))
            .find(|(_, r)| {
                !r.is_empty()
                    && if (r.start - cur).is_multiple_of(&2) {
                        r.start
                    } else {
                        r.start + 1
                    } <= r.end
            })
            .map(|(y, _)| y)?;

        // Find the maximum valid height.
        let max = intersect_range
            .iter()
            .rev()
            .filter(|y| (next - y).is_multiple_of(&2))
            .map(|y| (y, self.intersection(&Range::new(y - dist, y + dist))))
            .find(|(_, r)| {
                !r.is_empty()
                    && if (r.end - cur).is_multiple_of(&2) {
                        r.end
                    } else {
                        r.end - 1
                    } >= r.start
            })
            .map(|(y, _)| y)?;

        Some(Range::new(min, max))
    }
}

struct Reachables {
    cur: isize,
    ranges: Vec<Range>,
}

impl Reachables {
    fn new() -> Self {
        Self {
            cur: 0,
            ranges: vec![Range::new(0, 0)],
        }
    }

    // Find the next `Reachables` from my current set of `ranges` to the next `wall`'s openings.
    fn next(&self, wall: &Wall) -> Self {
        // For each of my ranges x the walls openings, find the reachable areas.
        let ranges: Vec<Range> = self
            .ranges
            .iter()
            .cartesian_product(wall.gaps.iter())
            .filter_map(|(prev_range, opening)| prev_range.reachable(opening, self.cur, wall.dist))
            .fold(Vec::new(), |mut acc, next_range| {
                match acc.last_mut() {
                    Some(last_range) if next_range.start <= last_range.end + 1 => {
                        last_range.end = last_range.end.max(next_range.end);
                    }
                    _ => {
                        acc.push(next_range);
                    }
                }
                acc
            });

        Self {
            cur: wall.dist,
            ranges,
        }
    }

    // The solution is (x+y)/2 where y the smallest value in the smallest reachable range.
    fn solution(&self) -> isize {
        (self.cur + self.ranges[0].start) / 2
    }
}

fn p1(input: &str) -> isize {
    // Now we can just go from one `Reachables` to another and then find the `solution()` from the last one.
    Wall::layout(parse(input))
        .iter()
        .fold(Reachables::new(), |acc, wall| acc.next(wall))
        .solution()
}

fn p2(input: &str) -> isize {
    p1(input)
}

fn p3(input: &str) -> isize {
    p1(input)
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
        let input = "7,7,2\n12,0,4\n15,5,3\n24,1,6\n28,5,5\n40,8,2";
        assert_eq!(p1(input), 24);
    }

    #[test]
    fn test_p2() {
        let input = "7,7,2\n7,1,3\n12,0,4\n15,5,3\n24,1,6\n28,5,5\n40,3,3\n40,8,2";
        assert_eq!(p2(input), 22);
    }

    #[test]
    fn test_p3() {
        let input = "7,7,2\n7,1,3\n12,0,4\n15,5,3\n24,1,6\n28,5,5\n40,3,3\n40,8,2";
        assert_eq!(p3(input), 22);
    }
}
