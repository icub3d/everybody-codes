use std::time::Instant;

use rustc_hash::FxHashMap;

const INPUT_PART1: &str = include_str!("inputs/quest06-1.txt");
type InputPart1<'a> = FxHashMap<&'a str, Vec<&'a str>>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input
        .lines()
        .map(|l| {
            let (k, vv) = l.split_once(':').unwrap();
            (k, vv.split(',').collect())
        })
        .collect()
}

// bfs to find all paths that end in '@'.
fn bfs<'a>(
    t: &FxHashMap<&'a str, Vec<&'a str>>,
    found: &mut Vec<Vec<&'a str>>,
    cur: &'a str,
    mut prefix: Vec<&'a str>,
) {
    if cur == "@" {
        found.push(prefix.clone());
        return;
    }
    for child in t.get(cur).unwrap_or(&vec![]) {
        prefix.push(child);
        bfs(t, found, child, prefix.clone());
        prefix.pop();
    }
}

fn p1(input: &InputPart1) -> String {
    let mut found = Vec::new();
    bfs(input, &mut found, "RR", vec![]);

    // Map paths (index) to their len.
    let counts = found.iter().enumerate().fold(
        FxHashMap::default(),
        |mut acc: FxHashMap<usize, Vec<usize>>, (i, f)| {
            acc.entry(f.len()).or_default().push(i);
            acc
        },
    );

    // Return the one with the unique path.
    "RR".to_owned()
        + &counts
            .iter()
            .find(|(_, vv)| vv.len() == 1)
            .map(|(_, vv)| found[vv[0]].join(""))
            .unwrap()
}

const INPUT_PART2: &str = include_str!("inputs/quest06-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

// Create the parent mapping as well as track all the "leaves" (they have an @ as their child).
fn get_leaves_and_parents<'a>(
    input: &'a InputPart2,
) -> (Vec<&'a str>, FxHashMap<&'a str, &'a str>) {
    input.iter().fold(
        (Vec::new(), FxHashMap::default()),
        |(mut leaves, mut parents), (parent, children)| {
            for child in children {
                if *child == "@" {
                    leaves.push(parent);
                    continue;
                }
                parents.insert(child, parent);
            }
            (leaves, parents)
        },
    )
}

// For each leaf, find it's path to the parent. If buggy, exclude @'s that have a bug or ant along
// the path.
fn find_leaf_paths<'a>(
    leaves: &[&'a str],
    parents: &FxHashMap<&'a str, &'a str>,
    buggy: bool,
) -> Vec<Vec<&'a str>> {
    leaves
        .iter()
        .filter_map(|mut leaf| {
            // We basically just go up the parent tree until we get RR or buggy.
            let mut path = vec!["@", *leaf];
            while let Some(parent) = parents.get(leaf) {
                path.push(*parent);
                leaf = parent;
                if *parent == "RR" {
                    continue;
                } else if (*parent == "ANT" || *parent == "BUG") && buggy {
                    return None;
                }
            }
            // Remember to reverse the path because we've gone up.
            Some(path.into_iter().rev().collect::<Vec<&str>>())
        })
        .collect()
}

// Find the path that's unique (similar to p1) but return just the first character.
fn find_unique(paths: &[Vec<&str>]) -> String {
    // Put the index of each path in the bucket of similar lengths.
    let counts = paths.iter().enumerate().fold(
        FxHashMap::default(),
        |mut acc: FxHashMap<usize, Vec<usize>>, (i, f)| {
            acc.entry(f.len()).or_default().push(i);
            acc
        },
    );

    // Turn them into string (p2 and p3 just take first char).
    counts
        .iter()
        .find(|(_, vv)| vv.len() == 1)
        .map(|(_, vv)| {
            paths[vv[0]]
                .iter()
                .map(|n| n.chars().next().unwrap())
                .collect::<String>()
        })
        .unwrap()
}

fn p2(input: &InputPart2) -> String {
    let (leaves, parents) = get_leaves_and_parents(input);
    let paths = find_leaf_paths(&leaves, &parents, false);
    find_unique(&paths)
}

const INPUT_PART3: &str = include_str!("inputs/quest06-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

fn p3(input: &InputPart3) -> String {
    let (leaves, parents) = get_leaves_and_parents(input);
    let paths = find_leaf_paths(&leaves, &parents, true);
    find_unique(&paths)
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
