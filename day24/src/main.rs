use file;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i64 {
    let mut result: i64 = 0;
    let mut is_first_part = true;
    let mut nodes_done: HashMap<String, usize> = HashMap::new();
    let mut to_do: HashSet<(String, String, String, String)> = HashSet::new();
    for l in lines {
        if l.len() == 0 {
            is_first_part = false;
            continue;
        }
        if is_first_part == true {
            let node;
            let val_str;
            (node, val_str) = l
                .split(": ")
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap();
            nodes_done.insert(node, val_str.parse().unwrap());
        } else {
            let left;
            let right;
            let l1;
            let l2;
            let op_str;
            (left, right) = l
                .split(" -> ")
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap();

            (l1, op_str, l2) = left
                .split(" ")
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap();
            to_do.insert((l1, l2, op_str, right));
        }
    }
    println!("{:?}", to_do);
    while to_do.len() > 0 {
        for next in &to_do.clone() {
            println!("next: {} {}", next.0, next.1);
            if nodes_done.contains_key(&next.0) && nodes_done.contains_key(&next.1) {
                println!("to_do.len {}", to_do.len());
                nodes_done.insert(
                    next.3.clone(),
                    match next.2.as_str() {
                        "AND" => {
                            (*nodes_done.get(&next.0).unwrap() == 1
                                && *nodes_done.get(&next.1).unwrap() == 1)
                                as usize
                        }
                        "OR" => {
                            (*nodes_done.get(&next.0).unwrap() == 1
                                || *nodes_done.get(&next.1).unwrap() == 1)
                                as usize
                        }
                        "XOR" => {
                            (*nodes_done.get(&next.0).unwrap() ^ *nodes_done.get(&next.1).unwrap())
                                as usize
                        }

                        _ => 0,
                    },
                );
                println!("{}", to_do.remove(&next));
            }
        }
    }
    for (n, val) in nodes_done {
        if n[0..1] == *"z" {
            println!("{}", n);
            if val == 1 {
                result += 2i64.pow(n[1..].parse::<u32>().unwrap());
            }
        }
    }

    //    println!("{:?}", nodes_done);
    result
}

fn solve2(lines: &Vec<String>) -> i64 {
    let mut result: i32 = 0;
    let mut result: i64 = 0;
    let mut is_first_part = true;
    let mut nodes_done: HashMap<String, usize> = HashMap::new();
    let mut to_do: HashSet<(String, String, String, String)> = HashSet::new();
    for l in lines {
        if l.len() == 0 {
            is_first_part = false;
            continue;
        }
        if is_first_part == true {
            let node;
            let val_str;
            (node, val_str) = l
                .split(": ")
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap();
            nodes_done.insert(node, val_str.parse().unwrap());
        } else {
            let left;
            let right;
            let l1;
            let l2;
            let op_str;
            (left, right) = l
                .split(" -> ")
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap();

            (l1, op_str, l2) = left
                .split(" ")
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap();
            to_do.insert((l1, l2, op_str, right));
        }
    }

    result
}
