use std::collections::HashMap;
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
    let input: String = fs::read_to_string(filename).unwrap();

    let steps: isize = 100;

    println!(
        "Part 1: After {} Steps there are {} flashes",
        steps,
        part_1(&input, steps)
    );
    println!(
        "Part 2: After {} steps all octopuses will flash simultaneously.",
        part_2(&input)
    );
}

fn part_1(input: &String, steps: isize) -> isize {
    let mut map = gen_map(input);
    let mut flashes = 0;
    
    for _i in 0..steps {
        let mut flash_queue: Vec<Coordinate> = Vec::new();
        for (coords, energy) in map.iter_mut() {
            *energy += 1;
            if *energy > 9 {
                flash_queue.push(*coords);
            }
        }
        while flash_queue.len() > 0 {
            let coords = flash_queue.pop().unwrap();
            flashes += 1;
            for c in get_neighboors(&coords) {
                if map.contains_key(&c) {
                    *map.entry(c).or_default() += 1;
                    if *map.get(&c).unwrap_or(&99) == 10 {
                        flash_queue.push(c);
                    }
                }
            }
        }

        for (_coords, energy) in map.iter_mut() {
            if *energy > 9 {
                *energy = 0;
            }
        }
    }

    return flashes;
}

fn part_2(input: &String) -> usize {
    let mut map = gen_map(input);
    let size = map.len();
    let mut step_counter: usize = 0;

    loop {
        step_counter += 1;
        let mut flash_queue: Vec<Coordinate> = Vec::new();
        for (coords, energy) in map.iter_mut() {
            *energy += 1;
            if *energy > 9 {
                flash_queue.push(*coords);
            }
        }
        while flash_queue.len() > 0 {
            let coords = flash_queue.pop().unwrap();
            for c in get_neighboors(&coords) {
                if map.contains_key(&c) {
                    *map.entry(c).or_default() += 1;
                    if *map.get(&c).unwrap_or(&99) == 10 {
                        flash_queue.push(c);
                    }
                }
            }
        }
        
        let mut flashes: usize = 0;
        for (_coords, energy) in map.iter_mut() {
            if *energy > 9 {
                flashes += 1;
                *energy = 0;
            }
        }
        if flashes == size {
            break;
        }
    }

    return step_counter;
}

fn get_neighboors(c: &Coordinate) -> Vec<Coordinate> {
    return vec![
        Coordinate {
            x: c.x - 1,
            y: c.y - 1,
        },
        Coordinate { x: c.x - 1, y: c.y },
        Coordinate {
            x: c.x - 1,
            y: c.y + 1,
        },
        Coordinate { x: c.x, y: c.y - 1 },
        Coordinate { x: c.x, y: c.y + 1 },
        Coordinate {
            x: c.x + 1,
            y: c.y - 1,
        },
        Coordinate { x: c.x + 1, y: c.y },
        Coordinate {
            x: c.x + 1,
            y: c.y + 1,
        },
    ];
}

fn gen_map(input: &String) -> HashMap<Coordinate, isize> {
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

    return map;
}
