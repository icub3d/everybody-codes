use std::time::Instant;

use rustc_hash::FxHashMap;

const INPUT_PART1: &str = include_str!("inputs/quest07-1.txt");
type InputPart1<'a> = (Vec<&'a str>, FxHashMap<char, Vec<char>>);
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    let (names, rules) = input.split_once("\n\n").unwrap();

    let names = names.split(',').collect();

    let rules = rules
        .lines()
        .map(|l| {
            let (key, values) = l.split_once(" > ").unwrap();
            let values = values
                .split(',')
                .map(|v| v.chars().next().unwrap())
                .collect();
            (key.chars().next().unwrap(), values)
        })
        .fold(FxHashMap::default(), |mut rules, (key, values)| {
            rules.insert(key, values);
            rules
        });

    (names, rules)
}

fn valid_name(name: &str, rules: &FxHashMap<char, Vec<char>>) -> bool {
    let mut cc = name.chars();
    let mut prev = cc.next().unwrap();
    for cur in cc {
        if let Some(vv) = rules.get(&prev)
            && vv.contains(&cur)
        {
            prev = cur;
        } else {
            return false;
        }
    }

    true
}

fn p1((names, rules): &InputPart1) -> String {
    names
        .iter()
        .find(|name| valid_name(name, rules))
        .unwrap()
        .to_string()
}

fn p2((names, rules): &InputPart2) -> usize {
    names
        .iter()
        .zip(1..)
        .map(|(name, id)| if valid_name(name, rules) { id } else { 0 })
        .sum::<usize>()
}

// Essentially this is a BFS for all the valid lengths of the "tree" from the rule-set. We cache
// values we've seen before so we don't have to answer those trees again.
fn find_all_names(
    name: &mut Vec<char>,
    rules: &FxHashMap<char, Vec<char>>,
    cache: &mut FxHashMap<(char, usize), usize>,
) -> usize {
    // Terminal case: the name has gotten too big.
    if name.len() > 11 {
        return 0;
    }

    // Maintain a cache. If we've already parsed this position, we can just return the answer.
    if let Some(r) = cache.get(&(name[name.len() - 1], name.len())) {
        return *r;
    }

    // Check to see if this name is long enough.
    let mut result = if name.len() >= 7 { 1 } else { 0 };

    // Try all the names with the rules for the last character.
    for c in rules.get(&name[name.len() - 1]).unwrap_or(&vec![]) {
        name.push(*c);
        result += find_all_names(name, rules, cache);
        name.pop();
    }

    // Update our cache and return the results.
    cache.insert((name[name.len() - 1], name.len()), result);
    result
}

fn p3((names, rules): &InputPart3) -> usize {
    let mut cache = FxHashMap::default();
    names
        .iter()
        .filter(|name| {
            valid_name(name, rules)
                // This has the effect of pruning out names that would be solved by smaller names.
                && names
                    .iter()
                    .all(|other| *name == other || !name.starts_with(other))
        })
        .map(|name| find_all_names(&mut name.chars().collect(), rules, &mut cache))
        .sum::<usize>()
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

const INPUT_PART2: &str = include_str!("inputs/quest07-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest07-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = parse_input_part1(INPUT_PART1);
        assert_eq!(input, input);
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
