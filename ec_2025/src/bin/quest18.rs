use std::{ops::Index, time::Instant};

const INPUT_PART1: &str = include_str!("inputs/quest18-1.txt");
const INPUT_PART2: &str = include_str!("inputs/quest18-2.txt");
const INPUT_PART3: &str = include_str!("inputs/quest18-3.txt");

#[derive(Debug, Clone)]
enum Branch {
    Free(isize),
    Connected(isize, usize),
}

impl From<&str> for Branch {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[1] {
            "branch" => Self::Connected(parts[7].parse().unwrap(), parts[4].parse().unwrap()),
            "free" => Self::Free(parts[5].parse().unwrap()),
            _ => panic!("unknown branch type"),
        }
    }
}

#[derive(Debug, Clone)]
struct Plant {
    thickness: isize,
    branches: Vec<Branch>,
}

impl Plant {
    fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let header = lines.next().unwrap();
        let parts: Vec<&str> = header.trim_end_matches(':').split_whitespace().collect();
        let thickness = parts[4].parse().unwrap();
        let branches = lines.map(Branch::from).collect();
        Self {
            thickness,
            branches,
        }
    }

    fn energy(&self, plants: &[Plant]) -> isize {
        let incoming: isize = self
            .branches
            .iter()
            .map(|b| match b {
                Branch::Free(thickness) => *thickness,
                Branch::Connected(thickness, branch) => {
                    plants[branch - 1].energy(plants) * thickness
                }
            })
            .sum();

        if incoming < self.thickness {
            0
        } else {
            incoming
        }
    }
}

struct Garden {
    plants: Vec<Plant>,
    tests: Vec<Vec<bool>>,
}

impl Garden {
    fn parse(input: &str) -> Self {
        let mut parts = input.split("\n\n\n");
        let plants: Vec<Plant> = parts
            .next()
            .unwrap()
            .split("\n\n")
            .map(Plant::parse)
            .collect();
        let tests = parts
            .next()
            .map(|s| {
                s.lines()
                    .map(|line| line.split_whitespace().map(|c| c == "1").collect())
                    .collect()
            })
            .unwrap_or_default();
        Self { plants, tests }
    }

    fn energy(&self) -> isize {
        self.plants.last().unwrap().energy(&self.plants)
    }

    fn energy_test(&self, test: &[bool]) -> isize {
        self.energy_test_helper(self.len() - 1, test)
    }

    fn energy_test_helper(&self, i: usize, test: &[bool]) -> isize {
        let thickness = if i < test.len() {
            return if test[i] { 1 } else { 0 };
        } else {
            self[i].thickness
        };

        let v: isize = self[i]
            .branches
            .iter()
            .map(|b| match b {
                Branch::Free(thickness) => *thickness,
                Branch::Connected(thickness, branch) => {
                    self.energy_test_helper(branch - 1, test) * thickness
                }
            })
            .sum();

        if v < thickness { 0 } else { v }
    }

    fn len(&self) -> usize {
        self.plants.len()
    }
}

impl Index<usize> for Garden {
    type Output = Plant;
    fn index(&self, i: usize) -> &Plant {
        &self.plants[i]
    }
}

fn p1(input: &str) -> isize {
    Garden::parse(input).energy()
}

fn p2(input: &str) -> isize {
    let garden = Garden::parse(input);
    garden.tests.iter().map(|t| garden.energy_test(t)).sum()
}

fn p3(input: &str) -> isize {
    let garden = Garden::parse(input);

    let optimal: Vec<bool> = (0..garden.tests[0].len())
        .map(|i| {
            (garden.tests[0].len()..garden.len())
                .flat_map(|plant_idx| &garden[plant_idx].branches)
                .filter_map(|branch| {
                    if let Branch::Connected(thickness, source_plant) = branch
                        && *source_plant - 1 == i
                    {
                        return Some(*thickness);
                    }
                    None
                })
                .sum::<isize>()
                > 0
        })
        .collect();

    let max = garden.energy_test(&optimal);

    garden
        .tests
        .iter()
        .map(|t| garden.energy_test(t))
        .filter(|energy| *energy > 0)
        .map(|energy| max - energy)
        .sum()
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
        let input = "Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 17:
- branch to Plant 1 with thickness 15
- branch to Plant 2 with thickness 3

Plant 5 with thickness 24:
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13

Plant 6 with thickness 15:
- branch to Plant 3 with thickness 14

Plant 7 with thickness 10:
- branch to Plant 4 with thickness 15
- branch to Plant 5 with thickness 21
- branch to Plant 6 with thickness 34";
        assert_eq!(p1(input), 774);
    }

    #[test]
    fn test_p2() {
        let input = "Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 10:
- branch to Plant 1 with thickness -25
- branch to Plant 2 with thickness 17
- branch to Plant 3 with thickness 12

Plant 5 with thickness 14:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -26
- branch to Plant 3 with thickness 15

Plant 6 with thickness 150:
- branch to Plant 4 with thickness 5
- branch to Plant 5 with thickness 6


1 0 1
0 0 1
0 1 1";
        assert_eq!(p2(input), 324);
    }

    #[test]
    fn test_p3() {
        let input = "Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 1:
- free branch with thickness 1

Plant 5 with thickness 8:
- branch to Plant 1 with thickness 11
- branch to Plant 2 with thickness 13
- branch to Plant 3 with thickness 9

Plant 6 with thickness 7:
- branch to Plant 4 with thickness -14
- branch to Plant 4 with thickness -9

Plant 7 with thickness 23:
- branch to Plant 5 with thickness 17
- branch to Plant 6 with thickness 18


0 1 0 0
0 1 0 1
1 1 1 0";
        assert_eq!(p3(input), 680);
    }
}
