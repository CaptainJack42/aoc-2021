use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coordinate {
    x: isize,
    y: isize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    solution(&args[1]);
}

fn solution(filename: &String) {
    println!("Filename: {}", filename);
    let input = fs::read_to_string(filename).unwrap();
    let mut map: HashMap<Coordinate, isize> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(
                Coordinate {
                    x: x as isize,
                    y: y as isize,
                },
                String::from(c).parse::<isize>().unwrap(),
            );
        }
    }
    println!("Part 1: cummulative danger level: {}", part_1(&map));
    println!("Part 2: Product of the 3 largest basins: {}", part_2(map));
}

fn part_1(map: &HashMap<Coordinate, isize>) -> isize {
    let mut danger_level: isize = 0;
    for (coords, height) in map {
        let mut is_low_point: bool = true;
        for n in get_neighboors(coords) {
            if map.get(&n).unwrap_or(&isize::MAX) <= height {
                is_low_point = false;
                break;
            }
        }
        if is_low_point {
            danger_level += 1 + height;
        }
    }
    return danger_level;
}

fn part_2(map: HashMap<Coordinate, isize>) -> isize {
    let mut basins: Vec<Coordinate> = Vec::new();
    for coords in map.keys() {
        let mut is_low_point: bool = true;
        let height = map.get(&coords).unwrap();
        for n in get_neighboors(&coords) {
            if map.get(&n).unwrap_or(&isize::MAX) <= height {
                is_low_point = false;
                break;
            }
        }
        if is_low_point {
            basins.push(*coords);
        }
    }

    return discover_basin(map, &basins);
}

fn discover_basin(mut map: HashMap<Coordinate, isize>, basins: &Vec<Coordinate>) -> isize {
    let mut sizes: Vec<isize> = Vec::new();
    for basin in basins {
        let mut considered: VecDeque<Coordinate> = VecDeque::new();
        considered.push_back(*basin);
        let mut size = 0;
        while considered.len() > 0 {
            let point = considered.pop_front().unwrap();
            if map.contains_key(&point) {
                size += 1;
                let curr_height = map.remove(&point).unwrap();
                for n in get_neighboors(&point) {
                    let height = *map.get(&n).unwrap_or(&isize::MAX);
                    if height > curr_height && height != 9 {
                        considered.push_back(n);
                    }
                }
            }
        }
        sizes.push(size);
    }

    sizes.sort_by(|a, b| b.cmp(a));
    return sizes[0] * sizes[1] * sizes[2];
}

fn get_neighboors(c: &Coordinate) -> Vec<Coordinate> {
    return vec![
        Coordinate { x: c.x - 1, y: c.y },
        Coordinate { x: c.x + 1, y: c.y },
        Coordinate { x: c.x, y: c.y + 1 },
        Coordinate { x: c.x, y: c.y - 1 },
    ];
}
