use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

const INPUT_PART1: &str = include_str!("inputs/quest17-1.txt");
const INPUT_PART2: &str = include_str!("inputs/quest17-2.txt");
const INPUT_PART3: &str = include_str!("inputs/quest17-3.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    row: isize,
    col: isize,
}

impl Tile {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // Get all neighbors of this tile that are within the grid.
    fn neighbors(&self, max_row: isize, max_col: isize) -> impl Iterator<Item = Tile> {
        Self::NEIGHBORS
            .into_iter()
            .map(|(dr, dc)| Tile::new(self.row + dr, self.col + dc))
            .filter(move |tile| {
                tile.row >= 0 && tile.row < max_row && tile.col >= 0 && tile.col < max_col
            })
    }

    // Get a quadrant for a tile based on where it is with respect to the center.
    fn quadrant(&self, center: &Tile) -> isize {
        if self.row <= center.row && self.col > center.col {
            3 // TopRight
        } else if self.row > center.row && self.col >= center.col {
            2 // BottomRight
        } else if self.row >= center.row && self.col < center.col {
            1 // BottomLeft
        } else {
            0 // TopLeft
        }
    }
}

struct Grid {
    tiles: Vec<Vec<char>>,
    volcano: Tile,
    start: Tile,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut volcano = Tile::new(0, 0);
        let mut start = Tile::new(0, 0);

        // Create the grid, but also track the volcano and start location.
        let cells: Vec<Vec<char>> = (0isize..)
            .zip(input.lines())
            .map(|(row, line)| {
                (0isize..)
                    .zip(line.chars())
                    .map(|(col, ch)| {
                        if ch == '@' {
                            volcano = Tile::new(row, col);
                        } else if ch == 'S' {
                            start = Tile::new(row, col);
                        }
                        ch
                    })
                    .collect()
            })
            .collect();

        Self {
            tiles: cells,
            volcano,
            start,
        }
    }

    fn height(&self) -> isize {
        self.tiles.len() as isize
    }

    fn width(&self) -> isize {
        self.tiles[0].len() as isize
    }

    fn max(&self) -> isize {
        self.height().min(self.width()) / 2
    }

    fn get(&self, pos: &Tile) -> char {
        self.tiles[pos.row as usize][pos.col as usize]
    }

    fn cost(&self, pos: &Tile) -> usize {
        match self.get(pos) {
            ch if ch.is_ascii_digit() => ch.to_digit(10).map(|d| d as usize).unwrap(),
            _ => 0,
        }
    }

    fn in_radius(&self, pos: &Tile, radius: isize) -> bool {
        let dr = (pos.row - self.volcano.row).abs();
        let dc = (pos.col - self.volcano.col).abs();
        dr * dr + dc * dc <= radius * radius
    }

    fn destruction(&self, radius: isize) -> usize {
        // Go through the bounding box of the radius but only include values that are "in_radius".
        (-radius..=radius)
            .cartesian_product(-radius..=radius)
            .filter(|&(dr, dc)| dr != 0 || dc != 0)
            .map(|(dr, dc)| Tile::new(self.volcano.row + dr, self.volcano.col + dc))
            .filter(|pos| {
                pos.row >= 0
                    && pos.row < self.height()
                    && pos.col >= 0
                    && pos.col < self.width()
                    && self.in_radius(pos, radius)
            })
            .map(|pos| self.cost(&pos))
            .sum()
    }
}

fn p1(input: &str) -> usize {
    // Just count destruction at radius 10.
    Grid::parse(input).destruction(10)
}

fn p2(input: &str) -> usize {
    let grid = Grid::parse(input);

    // Get the destruction at each radius and find the largest. We make windows because the
    // destruction of the current radius shouldn't include previous radii.
    let (radius, destruction) = (1..=grid.max())
        .map(|radius| (radius, grid.destruction(radius)))
        .tuple_windows()
        .map(|((_, d1), (r2, d2))| (r2, d2 - d1))
        .max_by_key(|&(_, destruction)| destruction)
        .unwrap();

    radius as usize * destruction
}

impl Grid {
    // Get the obstacles for the given grid to filter our neighbors.
    fn obstacles(&self, radius: isize) -> FxHashSet<Tile> {
        (0..self.height())
            .flat_map(|row| (0..self.width()).map(move |col| Tile::new(row, col)))
            .filter(|pos| self.in_radius(pos, radius))
            .collect()
    }
}

// This will track our status of making the loop as well as our current position.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct LoopState {
    pos: Tile,
    winding: isize,
    quadrant: isize,
}

impl LoopState {
    fn new(pos: Tile, center: &Tile) -> Self {
        Self {
            pos,
            winding: 0,
            quadrant: pos.quadrant(center),
        }
    }

    // Determine if we'd move from one quadrant to another.
    fn quadrant_delta(cur: isize, next: isize) -> isize {
        match (next - cur).rem_euclid(4) {
            1 => 1,  // Clockwise
            3 => -1, // Counter-clockwise
            _ => 0,  // Same in this case, across in general case as well.
        }
    }

    // Create a new state from moving to a neighbor position.
    fn next_state(&self, neighbor: Tile, center: &Tile) -> Self {
        let quadrant = neighbor.quadrant(center);
        Self {
            pos: neighbor,
            winding: self.winding + Self::quadrant_delta(self.quadrant, quadrant),
            quadrant,
        }
    }

    // If we've gone +/- 4, that's a full loop around the grid.
    fn winding_complete(&self) -> bool {
        self.winding.abs() >= 4
    }
}

// This is the node that our Dijkstra's algorithm will track.
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: usize,
    state: LoopState,
    prev: Tile,
}

impl Node {
    fn new(cost: usize, state: LoopState, prev: Tile) -> Self {
        Self { cost, state, prev }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_loop(grid: &Grid, lava: &FxHashSet<Tile>) -> Option<usize> {
    // Track our nodes to visit, prioritizing lower costs. Start with neighbors of `start`.
    let mut frontier = grid
        .start
        .neighbors(grid.height(), grid.width())
        .filter(|n| !lava.contains(n))
        .map(|n| Node::new(grid.cost(&n), LoopState::new(n, &grid.volcano), grid.start))
        .collect::<BinaryHeap<_>>();

    // Track our known distances
    let mut distances = FxHashMap::<LoopState, usize>::default();

    // Grab the lowest cost node until we find the shortest path or run out.
    while let Some(Node { cost, state, prev }) = frontier.pop() {
        // If we've been here before, we only want to try it if we
        // have a new better cost.
        if let Some(&prev) = distances.get(&state)
            && cost >= prev
        {
            continue;
        }

        // Track our distance to this state.
        distances.insert(state, cost);

        // Explore neighbors.
        for neighbor in state.pos.neighbors(grid.height(), grid.width()) {
            // We check for a winning solution and return it if we have one.
            if neighbor == grid.start && state.winding_complete() {
                return Some(cost);
            }

            // We don't want to backtrack, explore start, or an obstacle.
            if neighbor == prev || neighbor == grid.start || lava.contains(&neighbor) {
                continue;
            }

            // Add our neighbor node to frontier.
            frontier.push(Node::new(
                cost + grid.cost(&neighbor),
                state.next_state(neighbor, &grid.volcano),
                state.pos,
            ));
        }
    }

    None
}

fn p3(input: &str) -> usize {
    let grid = Grid::parse(input);

    // We are going to try all radii and find the smallest one that can make a loop.
    (1..=grid.max())
        .into_par_iter()
        .filter_map(|radius| Some((radius, find_shortest_loop(&grid, &grid.obstacles(radius))?)))
        .filter(|&(radius, time)| time / 30 <= radius as usize)
        .min_by_key(|&(radius, _)| radius)
        .map(|(radius, time)| time * radius as usize)
        .unwrap()
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
        let input = "189482189843433862719\n279415473483436249988\n432746714658787816631\n428219317375373724944\n938163982835287292238\n627369424372196193484\n539825864246487765271\n517475755641128575965\n685934212385479112825\n815992793826881115341\n1737798467@7983146242\n867597735651751839244\n868364647534879928345\n519348954366296559425\n134425275832833829382\n764324337429656245499\n654662236199275446914\n317179356373398118618\n542673939694417586329\n987342622289291613318\n971977649141188759131";
        assert_eq!(p1(input), 1573);
    }

    #[test]
    fn test_p2() {
        let input = "4547488458944\n9786999467759\n6969499575989\n7775645848998\n6659696497857\n5569777444746\n968586@767979\n6476956899989\n5659745697598\n6874989897744\n6479994574886\n6694118785585\n9568991647449";
        assert_eq!(p2(input), 1090);
    }

    #[test]
    fn test_p3() {
        let input = "2645233S5466644\n634566343252465\n353336645243246\n233343552544555\n225243326235365\n536334634462246\n666344656233244\n6426432@2366453\n364346442652235\n253652463426433\n426666225623563\n555462553462364\n346225464436334\n643362324542432\n463332353552464";
        assert_eq!(p3(input), 592);
    }

    #[test]
    fn test_p3_big() {
        let input = "545233443422255434324\n5222533434S2322342222\n523444354223232542432\n553522225435232255242\n232343243532432452524\n245245322252324442542\n252533232225244224355\n523533554454232553332\n522332223232242523223\n524523432425432244432\n3532242243@4323422334\n542524223994422443222\n252343244322522222332\n253355425454255523242\n344324325233443552555\n423523225325255345522\n244333345244325322335\n242244352245522323422\n443332352222535334325\n323532222353523253542\n553545434425235223552";
        assert_eq!(p3(input), 330);
    }

    #[test]
    fn test_p3_bigger() {
        let input = "5441525241225111112253553251553\n133522122534119S911411222155114\n3445445533355599933443455544333\n3345333555434334535435433335533\n5353333345335554434535533555354\n3533533435355443543433453355553\n3553353435335554334453355435433\n5435355533533355533535335345335\n4353545353545354555534334453353\n4454543553533544443353355553453\n5334554534533355333355543533454\n4433333345445354553533554555533\n5554454343455334355445533453453\n4435554534445553335434455334353\n3533435453433535345355533545555\n534433533533535@353533355553345\n4453545555435334544453344455554\n4353333535535354535353353535355\n4345444453554554535355345343354\n3534544535533355333333445433555\n3535333335335334333534553543535\n5433355333553344355555344553435\n5355535355535334555435534555344\n3355433335553553535334544544333\n3554333535553335343555345553535\n3554433545353554334554345343343\n5533353435533535333355343333555\n5355555353355553535354333535355\n4344534353535455333455353335333\n5444333535533453535335454535553\n3534343355355355553543545553345";
        assert_eq!(p3(input), 3180);
    }
}
