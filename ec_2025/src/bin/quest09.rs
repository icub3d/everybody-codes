use std::time::Instant;

use rayon::prelude::*;
use rustc_hash::FxHashMap;

struct Sequence {
    id: usize,
    sequence: Vec<u8>,
}

impl From<&str> for Sequence {
    fn from(value: &str) -> Self {
        let (id, sequence) = value.split_once(':').unwrap();
        Self {
            id: id.parse::<usize>().unwrap(),
            sequence: sequence.bytes().collect(),
        }
    }
}

impl Sequence {
    // Verify if the two given sequences could be the parents of this sequence. They "cover" all
    // the values of this sequence.
    fn parents(&self, l: &Sequence, r: &Sequence) -> bool {
        l.sequence
            .iter()
            .zip(r.sequence.iter())
            .zip(self.sequence.iter())
            .all(|((p1, p2), c)| c == p1 || c == p2)
    }

    // Count the number of similar values in this sequence with the given sequence.
    fn similarity(&self, other: &Sequence) -> usize {
        self.sequence
            .iter()
            .zip(other.sequence.iter())
            .filter(|(a, b)| a == b)
            .count()
    }

    // Convert this sequence and the given sequence into a bitmask. Each value is turned on if both
    // values are the same.
    fn bitmask(&self, other: &Sequence) -> u128 {
        self.sequence
            .iter()
            .zip(other.sequence.iter())
            .enumerate()
            .fold(
                0u128,
                |mask, (i, (l, r))| if l == r { mask | 1 << i } else { mask },
            )
    }
}

const INPUT_PART1: &str = include_str!("inputs/quest09-1.txt");
type InputPart1<'a> = Vec<Sequence>;
fn parse_input_part1(input: &'_ str) -> InputPart1<'_> {
    input.lines().map(Sequence::from).collect()
}

fn p1(sequences: &InputPart1) -> usize {
    // Only three sequences, fairly easy just to check each possibility.
    if sequences[0].parents(&sequences[1], &sequences[2]) {
        sequences[0].similarity(&sequences[1]) * sequences[0].similarity(&sequences[2])
    } else if sequences[1].parents(&sequences[0], &sequences[2]) {
        sequences[1].similarity(&sequences[0]) * sequences[1].similarity(&sequences[2])
    } else {
        sequences[2].similarity(&sequences[1]) * sequences[2].similarity(&sequences[0])
    }
}

struct Relationship {
    child: usize,
    p1: usize,
    p2: usize,
    sim_p1: usize,
    sim_p2: usize,
}

// The goal here is to find relationships (still N^3) but to minimize the effort while finding
// those relationships. Calling parents() on pairs made it take a few seconds to run. If we do the
// work of calculation the overlap of each pair (N^2), then the N^3 loop is more efficient. At
// least for my case.
fn find_relationships(sequences: &[Sequence]) -> Vec<Relationship> {
    let n = sequences.len();
    if n == 0 {
        return vec![];
    }

    // They all fit into a u128, so we can use a bit mask to find matches.
    let mut similarities = vec![vec![0u128; n]; n];
    for i in 0..n {
        for j in i..n {
            let bitmask = sequences[i].bitmask(&sequences[j]);
            similarities[i][j] = bitmask;
            similarities[j][i] = bitmask;
        }
    }

    // our tests aren't 128 bits (bug had me going wild).
    let mask = match sequences[0].sequence.len() == 128 {
        true => u128::MAX,
        false => (1u128 << sequences[0].sequence.len()) - 1,
    };

    // Now we can test for lineage by doing some bitwise logic.
    (0..n)
        .into_par_iter()
        .flat_map(|p1| {
            let mut found_relationships = Vec::new();
            for p2 in (p1 + 1)..n {
                for (c, similarity) in similarities.iter().enumerate() {
                    if c == p1 || c == p2 {
                        continue;
                    }
                    // Find the bits missing from p1 and make sure p2 has them.
                    let missing = similarity[p1] ^ mask;
                    if (missing & similarity[p2]) == missing {
                        found_relationships.push(Relationship {
                            child: c,
                            p1,
                            p2,
                            sim_p1: similarity[p1].count_ones() as usize,
                            sim_p2: similarity[p2].count_ones() as usize,
                        });
                    }
                }
            }
            found_relationships
        })
        .collect()
}

fn p2(sequences: &InputPart2) -> usize {
    let relationships = find_relationships(sequences);
    relationships
        .into_par_iter()
        .map(|r| r.sim_p1 * r.sim_p2)
        .sum()
}

// In computer science, a disjoint-set data structure, also called a union–find data structure or
// merge–find set, is a data structure that stores a collection of disjoint (non-overlapping) sets.
// Equivalently, it stores a partition of a set into disjoint subsets. It provides operations for
// adding new sets, merging sets (replacing them with their union), and finding a representative
// member of a set. The last operation makes it possible to determine efficiently whether any two
// elements belong to the same set or to different sets.
struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    // Creates a new `DisjointSet` instance with `n` disjoint sets.
    //
    // Each element `i` (from `0` to `n-1`) is initially in its own set,
    // with itself as the parent and a size of 1.
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    // Finds the representative (root) of the set containing element `i`. This is recursive and
    // will continue until it's found the parent.
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }

        // Once we find the root, we can flatten tree. Note that this happens recursively so all
        // elements are linked to the root.
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    // Unions the sets containing elements `i` and `j`.
    //
    // We attach the root of the smaller tree to the root of the larger tree. This helps to keep
    // the trees flat and improves performance.
    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);

        // They are in the same tree already.
        if root_i == root_j {
            return;
        }

        // Find the smaller tree and attach it to the larger.
        let (p1, p2) = match self.size[root_i] < self.size[root_j] {
            true => (root_i, root_j),
            false => (root_j, root_i),
        };
        self.parent[p1] = p2;
        self.size[p2] += self.size[p1];
    }
}

fn p3(sequences: &InputPart3) -> usize {
    let relationships = find_relationships(sequences);
    // Fill up our disjoint set.
    let mut ds = DisjointSet::new(sequences.len());
    for r in relationships {
        ds.union(r.child, r.p1);
        ds.union(r.child, r.p2);
    }

    // We want to find the largest set and return it's value (sum of ids).
    let mut components: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
    for i in 0..sequences.len() {
        let root = ds.find(i);
        components.entry(root).or_default().push(sequences[i].id);
    }
    components
        .values()
        .max_by_key(|v| v.len())
        .unwrap_or(&vec![])
        .iter()
        .sum()
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

const INPUT_PART2: &str = include_str!("inputs/quest09-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> InputPart2<'_> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest09-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> InputPart3<'_> {
    parse_input_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = parse_input_part1(
            "1:CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG
2:CTTGAATTGGGCCGTTTACCTGGTTTAACCAT
3:CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG",
        );
        assert_eq!(p1(&input), 414);
    }

    #[test]
    fn test_p2() {
        let input = parse_input_part2(
            "1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG",
        );
        assert_eq!(p2(&input), 1245);
    }

    #[test]
    fn test_p3() {
        let input = parse_input_part1(
            "1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG",
        );
        assert_eq!(p3(&input), 12);

        let input = parse_input_part1(
            "1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG
8:GGCGTAAAGTATGGATGCTGGCTAGGCACCCG",
        );
        assert_eq!(p3(&input), 36);
    }
}
