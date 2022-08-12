use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::{env, fs};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    solution(&args[1]);
}

fn solution(filename: &String) {
    let input = fs::read_to_string(filename).unwrap();
    let mut map: Vec<Vec<usize>> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        map.push(vec![]);
        for (_x, ch) in line.chars().enumerate() {
            map[y].push(ch.to_string().parse::<usize>().unwrap());
        }
    }
    println!("Part 1: The lowest total risk is: {}", part_1(&input));
}

fn part_1(input: &String) -> usize {
    let mut map: HashMap<Coordinate, usize> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            map.insert(
                Coordinate { x: x + 1, y: y + 1},
                ch.to_string().parse::<usize>().unwrap(),
            );
            max_x = x;
        }
        max_y = y;
    }
    let mut adj_list: Vec<Vec<Edge>> = Vec::new();
    let node_counter = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            let mut curr: Vec<Edge> = Vec::new();
            if map.contains_key(&Coordinate{x: x + 1, y: y}){
                curr.push(Edge{node: (x + 1) * y, cost: *map.get(&Coordinate{x: x + 1, y: y}).unwrap()})
            }
            if map.contains_key(&Coordinate{x: x, y: y + 1}){
                curr.push(Edge{node: x * (y + 1), cost: *map.get(&Coordinate{x: x, y: y + 1}).unwrap()})
            }
            adj_list.push(curr);
        }
    }
    for (key, val) in &map {
    }
    

    return 0;
}

fn shortest_path(adj_list: &HashMap<usize, Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }
        for edge in adj_list.get(&position).unwrap() {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    return None;
}
