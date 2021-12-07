use std::cmp;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    solution(filename);
}

fn solution(filename: &String) {
    println!("In file {}", filename);
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let positions = input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let answer_1 = part_1(&positions);
    println!(
        "Part 1: They spend {} fuel aligning at position {}",
        answer_1.0, answer_1.1
    );

    let answer_2 = part_2(&positions);
    println!(
        "Part 2: They spend {} fuel aligning at position {}",
        answer_2.0, answer_2.1
    );
}

fn part_1(positions: &Vec<i32>) -> (i32, i32) {
    let max = positions.iter().max().expect("empty vector").clone();
    let min = positions.iter().min().expect("empty vector").clone();
    let mut cost: i32;
    let mut opt_cost: i32 = 0;
    let mut opt_pos: i32 = 0;

    for i in min..max + 1 {
        cost = 0;
        for pos in positions {
            cost += cmp::max(pos, &i) - cmp::min(pos, &i);
        }
        if opt_cost == 0 || cost < opt_cost {
            opt_cost = cost;
            opt_pos = i;
        }
    }
    return (opt_cost, opt_pos);
}

fn part_2(positions: &Vec<i32>) -> (i32, i32) {
    let max = positions.iter().max().expect("empty vector").clone();
    let min = positions.iter().min().expect("empty vector").clone();
    let mut cost: i32;
    let mut opt_cost: i32 = 0;
    let mut opt_pos: i32 = 0;

    for i in min..max + 1 {
        cost = 0;
        for pos in positions {
            let dist = cmp::max(pos, &i) - cmp::min(pos, &i);
            for i in 1..dist + 1 {
                cost += i
            }
        }
        if opt_cost == 0 || cost < opt_cost {
            opt_cost = cost;
            opt_pos = i;
        }
    }
    return (opt_cost, opt_pos);
}
