use std::collections::{HashMap, VecDeque};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    solution(&args[1]);
}

fn solution(filename: &String) {
    println!("Filename: {}", filename);
    let input = fs::read_to_string(filename).unwrap();
    let mut lines: VecDeque<&str> = input.lines().collect();
    let pol_templ = lines.pop_front().unwrap().to_string();
    lines.pop_front(); // pop the blank line
    let mut templ: Vec<String> = Vec::new();
    for c in pol_templ.chars() {
        templ.push(c.to_string());
    }
    let mut lookup: HashMap<String, String> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split("->").collect();
        lookup.insert(split[0].trim().to_string(), split[1].trim().to_string());
    }
    let steps: isize = 10;
    println!(
        "Part 1: After {} steps the answer is: {}",
        steps,
        part_1(steps, templ.clone(), &lookup)
    );
    let steps: isize = 40;
    println!(
        "Part 2: After {} steps the answer is: {}",
        steps,
        part_2(steps, templ.clone(), &lookup)
    );
}

fn part_1(steps: isize, mut template: Vec<String>, lookup: &HashMap<String, String>) -> isize {
    for _i in 0..steps {
        let mut next_step: Vec<String> = Vec::new();
        let temp_i = template.windows(2).collect::<Vec<_>>();
        next_step.push(template[0].clone());
        for n in temp_i {
            let key: String = String::new() + &n[0] + &n[1];
            next_step.push(lookup.get(&key).unwrap().to_string());
            next_step.push(n[1].clone());
        }
        template = next_step.clone();
    }
    let mut counter: HashMap<String, isize> = HashMap::new();
    for n in template {
        *counter.entry(n).or_insert(0) += 1;
    }
    let max = counter.iter().max_by_key(|entry| entry.1).unwrap().1;
    let min = counter.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    return max - min;
}

fn part_2(steps: isize, template: Vec<String>, lookup: &HashMap<String, String>) -> u64 {
    let mut counter: HashMap<String, u64> = HashMap::new();
    let start_i = template.windows(2).collect::<Vec<_>>();
    // FIXME: Better but still fails at step 23. strlen is 2^(n+1)+1 -> needs A LOT of memory
    for n in start_i {
        let mut curr: Vec<String> = vec![n[0].clone(), n[1].clone()];
        *counter.entry(n[0].clone()).or_insert(0) += 1;
        *counter.entry(n[1].clone()).or_insert(0) += 1;
        for _i in 0..steps {
            let mut next_step: Vec<String> = Vec::new();
            let tmp_i = curr.windows(2).collect::<Vec<_>>();
            next_step.push(curr[0].clone());
            for m in tmp_i {
                let key: String = String::new() + &m[0] + &m[1];
                let val = lookup.get(&key).unwrap().to_string();
                next_step.push(val.clone());
                *counter.entry(val).or_insert(0) += 1;
                next_step.push(m[1].clone());
            }
            curr = next_step.clone();
            println!("steps: {}; len: {}", _i, curr.len());
        }
    }
    let max = counter.iter().max_by_key(|entry| entry.1).unwrap().1;
    let min = counter.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    return max - min;
}
