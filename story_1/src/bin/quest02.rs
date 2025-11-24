// Unwrap-gate, LOL

use std::{str::FromStr, time::Instant};

use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::rc::Rc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
enum Error {
    #[error("parsing: {0}")]
    ParseError(&'static str),

    #[error("unknown instruction")]
    UnknownInstruction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    value: usize,
    symbol: char,
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(value: &str) -> anyhow::Result<Self, Self::Err> {
        let (_, n) = value.split_once('=').ok_or(Error::ParseError("no ="))?;

        let (value, symbol) = n
            .trim_matches(['[', ']'])
            .split_once(',')
            .ok_or(Error::ParseError("no ,"))?;

        Ok(Node {
            value: value.parse::<usize>().unwrap(),
            symbol: symbol
                .chars()
                .next()
                .ok_or(Error::ParseError("no symbol char"))?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Add(usize, Node, Node),
    Swap(usize),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pp = s.split_whitespace();

        match pp.next().ok_or(Error::ParseError("no instruction"))? {
            "ADD" => Ok(Instruction::Add(
                pp.next()
                    .ok_or(Error::ParseError("getting id"))?
                    .split_once("=")
                    .map(|(_, r)| r.parse::<usize>())
                    .ok_or(Error::ParseError("getting id value"))?
                    .map_err(|_| Error::ParseError("parsing id"))?,
                Node::from_str(pp.next().ok_or(Error::ParseError("no left node"))?)?,
                Node::from_str(pp.next().ok_or(Error::ParseError("no left node"))?)?,
            )),
            "SWAP" => Ok(Instruction::Swap(
                pp.next()
                    .ok_or(Error::ParseError("getting id"))?
                    .parse::<usize>()
                    .map_err(|_| Error::ParseError("parsing id"))?,
            )),
            _ => Err(Error::UnknownInstruction),
        }
    }
}

const INPUT_PART1: &str = include_str!("inputs/quest02-1.txt");
type InputPart1<'a> = Vec<Instruction>;
fn parse_input_part1(input: &'_ str) -> anyhow::Result<InputPart1<'_>> {
    input
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<Instruction>, Error>>()
        .map_err(anyhow::Error::from)
}

#[derive(Debug)]
struct TangledTreeNode {
    id: usize,
    node: Node,
    left: NodeLink,
    right: NodeLink,
}

// Note: We could Box<TangledTreeNode> here and then use unsafe to swap pointers.
type NodeHandle = Rc<RefCell<TangledTreeNode>>;
type NodeLink = Option<NodeHandle>;

impl TangledTreeNode {
    fn new(id: usize, node: Node) -> Self {
        Self {
            id,
            node,
            left: None,
            right: None,
        }
    }
}

struct TangledTree {
    left_root: NodeLink,
    right_root: NodeLink,
}

impl TangledTree {
    fn new() -> Self {
        Self {
            left_root: None,
            right_root: None,
        }
    }

    fn add(&mut self, id: usize, left: Node, right: Node) {
        Self::add_helper(&mut self.left_root, id, left);
        Self::add_helper(&mut self.right_root, id, right);
    }

    fn add_helper(cur: &mut NodeLink, id: usize, node: Node) {
        match cur {
            None => *cur = Some(Rc::new(RefCell::new(TangledTreeNode::new(id, node)))),
            Some(n) => {
                let mut n_borrow = n.borrow_mut();
                match n_borrow.node.value.cmp(&node.value) {
                    std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
                        Self::add_helper(&mut n_borrow.right, id, node)
                    }
                    std::cmp::Ordering::Less => Self::add_helper(&mut n_borrow.left, id, node),
                }
            }
        }
    }

    fn depths(&self) -> (FxHashMap<usize, Vec<char>>, FxHashMap<usize, Vec<char>>) {
        let mut left = FxHashMap::default();
        let mut right = FxHashMap::default();

        Self::depths_helper(&mut left, &self.left_root, 0);
        Self::depths_helper(&mut right, &self.right_root, 0);

        (left, right)
    }

    fn depths_helper(map: &mut FxHashMap<usize, Vec<char>>, cur: &NodeLink, depth: usize) {
        if let Some(n) = cur {
            let n_borrow = n.borrow();
            map.entry(depth).or_default().push(n_borrow.node.symbol);
            Self::depths_helper(map, &n_borrow.right, depth + 1);
            Self::depths_helper(map, &n_borrow.left, depth + 1);
        }
    }

    fn swap(&mut self, id: usize) {
        let node1 = Self::find_node(&self.left_root, id);
        let node2 = Self::find_node(&self.right_root, id);

        if let (Some(n1), Some(n2)) = (node1, node2) {
            let mut n1_borrow = n1.borrow_mut();
            let mut n2_borrow = n2.borrow_mut();
            std::mem::swap(&mut n1_borrow.node, &mut n2_borrow.node);
        }
    }

    fn swap_branches(&mut self, id: usize) {
        let mut nodes_to_swap = Vec::new();
        Self::find_all_nodes(&self.left_root, id, &mut nodes_to_swap);
        Self::find_all_nodes(&self.right_root, id, &mut nodes_to_swap);

        if nodes_to_swap.len() == 2 {
            let mut n1_borrow = nodes_to_swap[0].borrow_mut();
            let mut n2_borrow = nodes_to_swap[1].borrow_mut();
            std::mem::swap(&mut *n1_borrow, &mut *n2_borrow);
        }
    }

    fn find_node(cur: &NodeLink, id: usize) -> NodeLink {
        if let Some(n) = cur {
            if n.borrow().id == id {
                return Some(Rc::clone(n));
            }
            if let Some(found) = Self::find_node(&n.borrow().left, id) {
                return Some(found);
            }
            if let Some(found) = Self::find_node(&n.borrow().right, id) {
                return Some(found);
            }
        }
        None
    }

    fn find_all_nodes(cur: &NodeLink, id: usize, found: &mut Vec<NodeHandle>) {
        if let Some(n) = cur {
            if n.borrow().id == id {
                found.push(Rc::clone(n));
            }
            Self::find_all_nodes(&n.borrow().left, id, found);
            Self::find_all_nodes(&n.borrow().right, id, found);
        }
    }
}

fn p1(input: &InputPart1) -> String {
    let mut tt = TangledTree::new();

    for instruction in input {
        match instruction {
            Instruction::Add(id, l, r) => tt.add(*id, *l, *r),
            Instruction::Swap(id) => tt.swap(*id),
        }
    }

    let (ld, rd) = tt.depths();

    ld.iter()
        .max_by_key(|(_, v)| v.len())
        .map(|(_, v)| v.iter().collect::<String>())
        .unwrap_or(String::new())
        + &rd
            .iter()
            .max_by_key(|(_, v)| v.len())
            .map(|(_, v)| v.iter().collect::<String>())
            .unwrap_or(String::new())
}

fn p2(input: &InputPart2) -> String {
    p1(input)
}

fn p3(input: &InputPart3) -> String {
    let mut tt = TangledTree::new();

    for instruction in input {
        match instruction {
            Instruction::Add(id, l, r) => tt.add(*id, *l, *r),
            Instruction::Swap(id) => tt.swap_branches(*id),
        }
    }

    let (ld, rd) = tt.depths();

    let left_str = ld
        .iter()
        .max_by(|(depth_a, nodes_a), (depth_b, nodes_b)| {
            nodes_a
                .len()
                .cmp(&nodes_b.len())
                .then_with(|| depth_b.cmp(depth_a))
        })
        .map(|(_, v)| v.iter().collect::<String>())
        .unwrap_or_default();

    let right_str = rd
        .iter()
        .max_by(|(depth_a, nodes_a), (depth_b, nodes_b)| {
            nodes_a
                .len()
                .cmp(&nodes_b.len())
                .then_with(|| depth_b.cmp(depth_a))
        })
        .map(|(_, v)| v.iter().collect::<String>())
        .unwrap_or_default();

    left_str + &right_str
}

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let input = parse_input_part1(INPUT_PART1)?;
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part2(INPUT_PART2)?;
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let input = parse_input_part3(INPUT_PART3)?;
    let solution = p3(&input);
    println!("p3 {:?} {}", now.elapsed(), solution);

    Ok(())
}

const INPUT_PART2: &str = include_str!("inputs/quest02-2.txt");
type InputPart2<'a> = InputPart1<'a>;
fn parse_input_part2(input: &'_ str) -> anyhow::Result<InputPart2<'_>> {
    parse_input_part1(input)
}

const INPUT_PART3: &str = include_str!("inputs/quest02-3.txt");
type InputPart3<'a> = InputPart1<'a>;
fn parse_input_part3(input: &'_ str) -> anyhow::Result<InputPart3<'_>> {
    parse_input_part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() -> anyhow::Result<()> {
        let input = parse_input_part1(
            "ADD id=1 left=[10,A] right=[30,H]\nADD id=2 left=[15,D] right=[25,I]\nADD id=3 left=[12,F] right=[31,J]\nADD id=4 left=[5,B] right=[27,L]\nADD id=5 left=[3,C] right=[28,M]\nADD id=6 left=[20,G] right=[32,K]\nADD id=7 left=[4,E] right=[21,N]",
        )?;
        assert_eq!(p1(&input), "CFGNLK");
        Ok(())
    }

    #[test]
    fn test_p1_big() -> anyhow::Result<()> {
        let input = parse_input_part1(
            "ADD id=1 left=[160,E] right=[175,S]\nADD id=2 left=[140,W] right=[224,D]\nADD id=3 left=[122,U] right=[203,F]\nADD id=4 left=[204,N] right=[114,G]\nADD id=5 left=[136,V] right=[256,H]\nADD id=6 left=[147,G] right=[192,O]\nADD id=7 left=[232,I] right=[154,K]\nADD id=8 left=[118,E] right=[125,Y]\nADD id=9 left=[102,A] right=[210,D]\nADD id=10 left=[183,Q] right=[254,E]\nADD id=11 left=[146,E] right=[148,C]\nADD id=12 left=[173,Y] right=[299,S]\nADD id=13 left=[190,B] right=[277,B]\nADD id=14 left=[124,T] right=[142,N]\nADD id=15 left=[153,R] right=[133,M]\nADD id=16 left=[252,D] right=[276,M]\nADD id=17 left=[258,I] right=[245,P]\nADD id=18 left=[117,O] right=[283,!]\nADD id=19 left=[212,O] right=[127,R]\nADD id=20 left=[278,A] right=[169,C]",
        )?;
        assert_eq!(p1(&input), "EVERYBODYCODES");
        Ok(())
    }

    #[test]
    fn test_p2() -> anyhow::Result<()> {
        let input = parse_input_part2(
            "ADD id=1 left=[10,A] right=[30,H]\nADD id=2 left=[15,D] right=[25,I]\nADD id=3 left=[12,F] right=[31,J]\nADD id=4 left=[5,B] right=[27,L]\nADD id=5 left=[3,C] right=[28,M]\nSWAP 1\nSWAP 5\nADD id=6 left=[20,G] right=[32,K]\nADD id=7 left=[4,E] right=[21,N]",
        )?;
        assert_eq!(p2(&input), "MGFLNK");
        Ok(())
    }

    #[test]
    fn test_p3() -> anyhow::Result<()> {
        let input = parse_input_part3(
            "ADD id=1 left=[10,A] right=[30,H]\nADD id=2 left=[15,D] right=[25,I]\nADD id=3 left=[12,F] right=[31,J]\nADD id=4 left=[5,B] right=[27,L]\nADD id=5 left=[3,C] right=[28,M]\nSWAP 1\nSWAP 5\nADD id=6 left=[20,G] right=[32,K]\nADD id=7 left=[4,E] right=[21,N]\nSWAP 2",
        )?;
        assert_eq!(p3(&input), "DJMGL");
        Ok(())
    }

    #[test]
    fn test_p3_2() -> anyhow::Result<()> {
        let input = parse_input_part3(
            "ADD id=1 left=[10,A] right=[30,H]\nADD id=2 left=[15,D] right=[25,I]\nADD id=3 left=[12,F] right=[31,J]\nADD id=4 left=[5,B] right=[27,L]\nADD id=5 left=[3,C] right=[28,M]\nSWAP 1\nSWAP 5\nADD id=6 left=[20,G] right=[32,K]\nADD id=7 left=[4,E] right=[21,N]\nSWAP 2\nSWAP 5",
        )?;
        assert_eq!(p3(&input), "DJCGL");
        Ok(())
    }
}
