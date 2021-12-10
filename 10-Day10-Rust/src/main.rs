use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    solution(&args[1]);
}

fn solution(filename: &String) {
    println!("Filename: {}", filename);
    let input = fs::read_to_string(filename).unwrap();

    println!("Part 1: The syntax error score is: {}", part_1(&input));
    println!("Part 2: The autocompletion score is: {}", part_2(&input));
}

fn part_1(input: &String) -> isize {
    let mut illegal: HashMap<char, isize> = HashMap::from([(')', 0), (']', 0), ('}', 0), ('>', 0)]);
    let opening: Vec<char> = vec!['(', '[', '{', '<'];
    let closing: Vec<char> = vec![')', ']', '}', '>'];

    for line in input.lines() {
        let mut bracket_q: VecDeque<char> = VecDeque::new();
        for c in line.chars() {
            if opening.contains(&c) {
                bracket_q.push_front(c);
            } else if closing.contains(&c) {
                let opened = bracket_q.pop_front().unwrap();
                if opening[closing.iter().position(|&r| r == c).unwrap()] != opened {
                    *illegal.entry(c).or_default() += 1;
                    break;
                }
            }
        }
    }

    let mut score: isize = 0;
    score += illegal.get(&')').unwrap_or(&0) * 3;
    score += illegal.get(&']').unwrap_or(&0) * 57;
    score += illegal.get(&'}').unwrap_or(&0) * 1197;
    score += illegal.get(&'>').unwrap_or(&0) * 25137;

    return score;
}

fn part_2(input: &String) -> isize {
    let opening: Vec<char> = vec!['(', '[', '{', '<'];
    let closing: Vec<char> = vec![')', ']', '}', '>'];
    let score_lookup: HashMap<char, isize> =
        HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut scores: Vec<isize> = Vec::new();

    for line in input.lines() {
        let mut bracket_q: VecDeque<char> = VecDeque::new();
        let mut corrupt: bool = false;
        for c in line.chars() {
            if opening.contains(&c) {
                bracket_q.push_front(c);
            } else if closing.contains(&c) {
                let opened = bracket_q.pop_front().unwrap();
                if opening[closing.iter().position(|&r| r == c).unwrap()] != opened {
                    corrupt = true;
                    break;
                }
            }
        }
        if !corrupt {
            let mut tmp_score: isize = 0;
            while bracket_q.len() > 0 {
                let tb_closed = bracket_q.pop_front().unwrap();
                tmp_score = tmp_score * 5 + *score_lookup.get(&tb_closed).unwrap();
            }
            scores.push(tmp_score);
        }
    }

    scores.sort_by(|a, b| b.cmp(a));
    let score: isize = scores[scores.len() / 2];
    return score;
}
