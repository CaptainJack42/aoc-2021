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
    let mut pair_counter: HashMap<String, u64> = HashMap::new();
    for n in &template {
        *counter.entry(n.clone()).or_insert(0) += 1;
    }
    for n in template.windows(2).collect::<Vec<_>>() {
        let key: String = String::new() + &n[0] + &n[1];
        *pair_counter.entry(key).or_insert(0) += 1;
    }
    for _i in 0..steps {
        let mut new: Vec<(String, String, u64)> = Vec::new();
        for (key, val) in lookup {
            if pair_counter.contains_key(key){
                new.push((key.clone(), val.clone(), *pair_counter.get(key).unwrap()));
                pair_counter.remove(key);
            }
        }
        for (pair, c, cnt) in new {
            *counter.entry(c.clone()).or_insert(0) += cnt;
            let pair = pair.chars().collect::<Vec<_>>();
            let p1 = String::new() + &pair[0].to_string() + &c;
            let p2 = String::new() + &c + &pair[1].to_string();
            *pair_counter.entry(p1).or_insert(0) += cnt;
            *pair_counter.entry(p2).or_insert(0) += cnt; 
        }
    }
    let max = counter.iter().max_by_key(|entry| entry.1).unwrap().1;
    let min = counter.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    return max - min;
}
