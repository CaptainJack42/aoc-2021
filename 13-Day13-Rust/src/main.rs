use std::collections::HashSet;
use std::{env, fs};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

struct Fold {
    along_y: bool,
    coord: isize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    solution(&args[1]);
}

fn solution(filename: &String) {
    println!("Filename: {}", filename);
    let input: String = fs::read_to_string(filename).unwrap();

    let mut points: HashSet<Coordinate> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();
    for line in input.lines() {
        if line == "\n" {
            continue;
        }
        if line.contains("fold") {
            let chunks: Vec<&str> = line.split("=").collect();
            let axis = &chunks[0].replace("fold along ", "");
            if axis == "y" {
                folds.push(Fold {
                    along_y: true,
                    coord: chunks[1].to_string().parse::<isize>().unwrap(),
                })
            } else {
                folds.push(Fold {
                    along_y: false,
                    coord: chunks[1].to_string().parse::<isize>().unwrap(),
                })
            }
        } else {
            let chunks: Vec<&str> = line.split(",").collect();
            if chunks == vec![""] {
                continue;
            }
            let c = Coordinate {
                x: chunks[0].to_string().parse::<isize>().unwrap(),
                y: chunks[1].to_string().parse::<isize>().unwrap(),
            };
            points.insert(c);
        }
    }

    println!(
        "Part 1: After {} fold there are still {} Points visible.",
        1,
        part_1(points.clone(), &folds[0])
    );
    println!(
        "Part 2: After {} folds there are still {} Points visible.",
        folds.len(),
        part_2(&points, &folds)
    )
}

fn part_1(points: HashSet<Coordinate>, fold: &Fold) -> isize {
    let mut new_points: HashSet<Coordinate> = HashSet::new();
    if fold.along_y {
        for p in points {
            if p.y > fold.coord {
                let new_y = 2 * fold.coord - p.y;
                if new_y >= 0 {
                    new_points.insert(Coordinate { x: p.x, y: new_y });
                }
            } else {
                new_points.insert(p);
            }
        }
    } else {
        for p in points {
            if p.x > fold.coord {
                let new_x = 2 * fold.coord - p.x;
                if new_x >= 0 {
                    new_points.insert(Coordinate { x: new_x, y: p.y });
                }
            } else {
                new_points.insert(p);
            }
        }
    }
    print_result(&new_points);
    return new_points.len() as isize;
}

fn part_2(points: &HashSet<Coordinate>, folds: &Vec<Fold>) -> isize {
    let mut new_points: HashSet<Coordinate> = points.clone();
    for fold in folds {
        let points = new_points.clone();
        if fold.along_y {
            for p in points {
                if p.y > fold.coord {
                    new_points.remove(&p);
                    let new_y = 2 * fold.coord - p.y;
                    if new_y >= 0 {
                        new_points.insert(Coordinate { x: p.x, y: new_y });
                    }
                } else {
                    new_points.insert(p);
                }
            }
        } else {
            for p in points {
                if p.x > fold.coord {
                    new_points.remove(&p);
                    let new_x = 2 * fold.coord - p.x;
                    if new_x >= 0 {
                        new_points.insert(Coordinate { x: new_x, y: p.y });
                    }
                } else {
                    new_points.insert(p);
                }
            }
        }
    }
    print_result(&new_points);
    return new_points.len() as isize;
}

fn print_result(points: &HashSet<Coordinate>) {
    let max_x: isize = points.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let max_y: isize = points.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if points.contains(&Coordinate{x: x, y: y}) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
