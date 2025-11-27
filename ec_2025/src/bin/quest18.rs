use std::{
    ops::{Index, IndexMut},
    time::Instant,
};

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
        let parts = s.split_whitespace().collect::<Vec<_>>();
        match parts[1] {
            "branch" => Self::Connected(parts[7].parse().unwrap(), parts[4].parse().unwrap()),
            "free" => Self::Free(parts[5].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Plant {
    thickness: isize,
    branches: Vec<Branch>,
}

impl From<&str> for Plant {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();
        let header = lines.next().unwrap();
        let parts = header
            .trim_end_matches(':')
            .split_whitespace()
            .collect::<Vec<_>>();
        let thickness = parts[4].parse().unwrap();
        let branches = lines.map(Branch::from).collect();
        Self {
            thickness,
            branches,
        }
    }
}

impl Plant {
    fn energy(&self, plants: &[Plant]) -> isize {
        // Find the incoming energy recursively.
        let incoming = self
            .branches
            .iter()
            .map(|b| match b {
                Branch::Free(thickness) => *thickness,
                Branch::Connected(thickness, branch) => {
                    plants[branch - 1].energy(plants) * thickness
                }
            })
            .sum::<isize>();

        if incoming < self.thickness {
            0
        } else {
            incoming
        }
    }
}

struct Garden {
    plants: Vec<Plant>,
}

impl IndexMut<usize> for Garden {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.plants[i]
    }
}

impl Index<usize> for Garden {
    type Output = Plant;
    fn index(&self, i: usize) -> &Plant {
        &self.plants[i]
    }
}

impl Garden {
    fn new(plants: Vec<Plant>) -> Self {
        Self { plants }
    }

    // p1 - what is the energy output of last plant.
    fn energy(&self) -> isize {
        self.plants.last().unwrap().energy(&self.plants)
    }

    fn energy_test(&mut self, test: &[isize]) -> isize {
        // Modify the initial plants and then run the logic from p1.
        test.iter()
            .enumerate()
            .for_each(|(i, t)| self[i].branches[0] = Branch::Free(*t));
        self.energy()
    }

    fn len(&self) -> usize {
        self.plants.len()
    }
}

fn parse(input: &str) -> (Garden, Vec<Vec<isize>>) {
    let mut parts = input.split("\n\n\n");
    let plants = parts
        .next()
        .unwrap()
        .split("\n\n")
        .map(Plant::from)
        .collect::<Vec<_>>();
    let tests = parts
        .next()
        .map(|s| {
            s.lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|c| c.parse::<isize>().unwrap())
                        .collect()
                })
                .collect()
        })
        .unwrap_or_default();
    (Garden::new(plants), tests)
}

fn p1(input: &str) -> isize {
    let (garden, _) = parse(input);
    garden.energy()
}

fn p2(input: &str) -> isize {
    let (mut garden, tests) = parse(input);
    tests.iter().map(|t| garden.energy_test(t)).sum()
}

// 2^81, lol, see you at the heat death of the universe.
fn p3(input: &str) -> isize {
    let (mut garden, tests) = parse(input);

    let free_branches = tests[0].len();

    // For each "input", find the connected nodes and sum them up. If it's positive, we want to include it.
    let optimal: Vec<isize> = (0..free_branches)
        .map(|i| {
            if (free_branches..garden.len())
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
            {
                1
            } else {
                0
            }
        })
        .collect();

    // Calculate the max.
    let max = garden.energy_test(&optimal);

    // Find the difference with the test cases where energy > 0
    tests
        .iter()
        .map(|t| garden.energy_test(t))
        .filter(|energy| *energy > 0)
        .map(|energy| max - energy)
        .sum()
}

fn p3_z3(input: &str) -> isize {
    use z3::{
        Optimize,
        ast::{Bool, Int},
    };

    let (mut garden, tests) = parse(input);
    let free_branches = tests[0].len();
    let opt = Optimize::new();
    let one = Int::from_i64(1);
    let zero = Int::from_i64(0);

    // Create boolean variables for each free branch
    let free_branch_vars: Vec<Bool> = (0..free_branches)
        .map(|i| Bool::new_const(format!("fb_{}", i)))
        .collect();

    // Create integer variables for each plant's energy.
    let plant_energies: Vec<Int> = (0..garden.len())
        .map(|i| Int::new_const(format!("plant_{}", i)))
        .collect();

    // For free branch plants, assert energy is 0 or 1.
    for i in 0..free_branches {
        let energy_one = plant_energies[i].eq(&one);
        let energy_zero = plant_energies[i].eq(&zero);
        opt.assert(&free_branch_vars[i].ite(&energy_one, &energy_zero));
    }

    // For other plants, calculate their energy based on branches
    for i in free_branches..garden.len() {
        let plant = &garden[i];

        // Calculate incoming energy
        let mut incoming = Int::from_i64(0);
        for branch in &plant.branches {
            match branch {
                Branch::Free(thickness) => {
                    incoming += Int::from_i64(*thickness as i64);
                }
                Branch::Connected(thickness, source_idx) => {
                    let source_energy = &plant_energies[source_idx - 1];
                    let contribution = source_energy * Int::from_i64(*thickness as i64);
                    incoming += contribution;
                }
            }
        }

        // Energy is incoming if incoming >= thickness, else 0
        let threshold = Int::from_i64(plant.thickness as i64);
        let activated = incoming.ge(&threshold);
        let zero = Int::from_i64(0);
        opt.assert(&activated.ite(
            &plant_energies[i].eq(&incoming),
            &plant_energies[i].eq(&zero),
        ));

        // Also ensure energy is non-negative
        opt.assert(&plant_energies[i].ge(&zero));
    }

    // Maximize the energy of the last plant
    let last_plant_energy = &plant_energies[garden.len() - 1];
    opt.maximize(last_plant_energy);

    // Check satisfiability and get maximum
    let max = if opt.check(&[]) == z3::SatResult::Sat {
        let model = opt.get_model().unwrap();
        model
            .eval(last_plant_energy, true)
            .unwrap()
            .as_i64()
            .unwrap() as isize
    } else {
        panic!("No solution found");
    };

    // Now just do the same as p3.
    tests
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

    let now = Instant::now();
    let solution = p3_z3(INPUT_PART3);
    println!("p3_z3 {:?} {}", now.elapsed(), solution);
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

    #[test]
    fn test_p3_z3() {
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
        assert_eq!(p3_z3(input), 680);
    }
}
