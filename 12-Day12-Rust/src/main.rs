use std::collections::{HashMap, VecDeque, HashSet};
use std::{env, fs};

#[derive(Debug, Clone)]
struct Path<'a> {
    current: &'a str,
    prev_caves: HashSet<&'a str>,
    small_visited: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    solution(&args[1]);
}

fn solution(filename: &String) {
    println!("Filename: {}", filename);
    let input: String = fs::read_to_string(filename).unwrap();

    println!(
        "Part 1: There are {} paths through this cave.",
        part_1(&input)
    );
    println!("Part 2: There are {} paths through this cave now.", part_2(&input));
}

fn part_1(input: &String) -> isize {
    let mut branches: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let mut chunks = line.split("-");
        let a = chunks.next().unwrap().to_string();
        let b = chunks.next().unwrap().to_string();
        branches
            .entry(a.clone())
            .and_modify(|n| n.push(b.clone()))
            .or_insert(vec![b.clone()]);
        branches
            .entry(b)
            .and_modify(|n| n.push(a.clone()))
            .or_insert(vec![a]);
    }

    let mut vis_q: VecDeque<Vec<String>> = VecDeque::new();
    let mut path_counter: isize = 0;
    vis_q.push_back(vec![String::from("start")]);
    while let Some(path) = vis_q.pop_front() {
        let last = path.last().unwrap();
        for n in branches.get(last).unwrap().iter() {
            if n.to_lowercase() == *n && path.contains(n) {
                continue;
            }
            if n == "end" {
                path_counter += 1;
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(n.clone());
            vis_q.push_back(new_path);
        }
    }
    return path_counter;
}

fn part_2(input: &String) -> isize {
    let mut branches: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut chunks = line.split("-");
        let a = chunks.next().unwrap();
        let b = chunks.next().unwrap();
        branches
            .entry(a)
            .and_modify(|n| n.push(b))
            .or_insert(vec![b]);
        branches
            .entry(b)
            .and_modify(|n| n.push(a))
            .or_insert(vec![a]);
    }

    let mut vis_q: VecDeque<Path> = VecDeque::new();
    let mut path_counter: isize = 0;
    vis_q.push_back(Path {
        current: "start",
        prev_caves: HashSet::new(),
        small_visited: false,
    });

    while let Some(path) = vis_q.pop_front() {
        for n in branches.get(&path.current).unwrap().iter() {
            if *n == "start" {
                continue;
            }
            if *n == "end" {
                path_counter += 1;
                continue;
            }
            let mut small_visited = path.small_visited;
            if n.to_lowercase() == *n {
                if path.prev_caves.contains(n) {
                    if path.small_visited {
                        continue;
                    }
                    small_visited = true;
                }
            }
            let mut new_path = Path {
                current: n,
                prev_caves: path.prev_caves.clone(),
                small_visited: small_visited,
            };
            new_path.prev_caves.insert(n);

            vis_q.push_back(new_path);
        }
    }
    return path_counter;
}