use std::{
    collections::VecDeque,
    ops::{Add, AddAssign, Mul},
    time::Instant,
};

use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use rustc_hash::FxHashSet;

const INPUT_PART1: &str = include_str!("inputs/quest15-1.txt");
type InputPart1<'a> = Vec<(char, isize)>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .trim()
        .split(',')
        .map(|v| {
            let (dir, dist) = v.split_at(1);
            (dir.chars().next().unwrap(), dist.parse::<isize>().unwrap())
        })
        .collect()
}

// Seemed easier for my to reason about points in this one.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// Scalar multiplication.
impl Mul<isize> for Point {
    type Output = Point;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn turn(&self, dir: char) -> Self {
        // We always switch x and y. If we turn left, we negate y, if turn right we negate x.
        match dir {
            'L' => Self {
                x: self.y,
                y: -self.x,
            },
            'R' => Self {
                x: -self.y,
                y: self.x,
            },
            _ => unreachable!(),
        }
    }
}

// For p1 and p2, I was able to just build all the walls and then return the last point as the end.
fn build_walls(input: &InputPart1) -> (FxHashSet<Point>, Point) {
    let mut walls = FxHashSet::default();
    let mut cur = Point::new(0, 0);
    let mut delta = Point::new(0, -1);

    for &(dir, dist) in input {
        // Turn.
        delta = delta.turn(dir);

        // Add each point along the way as a wall.
        (1isize..=dist).for_each(|v| {
            walls.insert(cur + delta * v);
        });

        // Update our current position.
        cur += delta * dist;
    }

    // Remove the end.
    walls.remove(&cur);

    (walls, cur)
}

fn p1(input: &InputPart1) -> usize {
    let (walls, end) = build_walls(input);

    // Since all distances are 1, we can simply bfs.
    let mut seen = FxHashSet::default();
    seen.insert(Point::default());
    let mut frontier = VecDeque::new();
    frontier.push_back((Point::default(), 0));

    while let Some((cur, dist)) = frontier.pop_front() {
        // Did we find the end?
        if cur == end {
            return dist;
        }

        for delta in [
            Point::new(0, 1),
            Point::new(0, -1),
            Point::new(-1, 0),
            Point::new(1, 0),
        ] {
            // Only add our neighbor if it's not a wall and we haven't seen it.
            let neighbor = cur + delta;
            if !walls.contains(&neighbor) && seen.insert(neighbor) {
                frontier.push_back((neighbor, dist + 1));
            }
        }
    }

    unreachable!()
}

fn p2(input: &InputPart2) -> usize {
    // Just called p1 for p2, LOL.
    p1(input)
}

impl Point {
    // For p3 we want the Manhattan distance.
    fn distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    // Get all 8 neighboring points. Instead of collecting, we can return the iterator.
    fn neighbors(&self) -> impl Iterator<Item = Point> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .map(|(dx, dy)| Point::new(dx, dy) + *self)
    }
}

struct Map {
    start: Point,
    end: Point,
    walls: Vec<(Point, Point)>,
    points_of_interest: FxHashSet<Point>,
}

impl Map {
    fn from_input(input: &InputPart1) -> Self {
        // We start by building our walls and tracking corners.
        let mut walls = Vec::new();
        let mut corners = Vec::new();
        let mut delta = Point::new(0, -1);
        let start = Point::default();
        let mut next = start;
        corners.push(start);

        for &(dir, dist) in input {
            delta = delta.turn(dir);
            next += delta;
            let begin = next;
            next += delta * (dist - 2);
            walls.push((begin, next));
            next += delta;
            corners.push(next);
        }

        // For each corner, we add it's non-wall neighbors as points of interest.
        let points_of_interest = corners
            .iter()
            .flat_map(|corner| corner.neighbors())
            .filter(|&point| Self::valid_move(&walls, point, point))
            .chain([start, next]) // Also add start and end as points of interest.
            .collect();

        Self {
            start,
            end: next,
            walls,
            points_of_interest,
        }
    }

    // Each neighbor is any point of interest that can be reached without crossing a wall.
    fn neighbors(&self, from: Point) -> Vec<(Point, usize)> {
        self.points_of_interest
            .iter()
            .copied()
            .filter(|&to| to != from && Self::valid_move(&self.walls, from, to))
            .map(|to| (to, from.distance(&to)))
            .collect()
    }

    // Check if movement from one point to another crosses any wall
    fn valid_move(walls: &[(Point, Point)], from: Point, to: Point) -> bool {
        // This is basically checking bounding boxes. from->to is a rectangle and the wall is a really thin rectangle.
        let x_min = from.x.min(to.x);
        let x_max = from.x.max(to.x);
        let y_min = from.y.min(to.y);
        let y_max = from.y.max(to.y);

        walls.iter().all(|&(w1, w2)| {
            let wx_min = w1.x.min(w2.x);
            let wx_max = w1.x.max(w2.x);
            let wy_min = w1.y.min(w2.y);
            let wy_max = w1.y.max(w2.y);
            x_min > wx_max || x_max < wx_min || y_min > wy_max || y_max < wy_min
        })
    }
}

fn p3(input: &InputPart3) -> usize {
    // Now we can just run Dijkstra's algorithm on the points of interest.
    let map = Map::from_input(input);
    let (_, cost) = dijkstra(
        &map.start,
        |&from| map.neighbors(from),
        |&pos| pos == map.end,
    )
    .unwrap();
    cost
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
    let solution: usize = p3(&input);
    println!("p3 {:?} {}", now.elapsed(), solution);
}

const INPUT_PART2: &str = include_str!("inputs/quest15-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest15-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input =
            parse_input_part1("L6,L3,L6,R3,L6,L3,L3,R6,L6,R6,L6,L6,R3,L3,L3,R3,R3,L6,L6,L3");
        assert_eq!(p1(&input), 16);
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
