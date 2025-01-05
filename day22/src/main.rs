use std::collections::{HashMap, HashSet};

use file;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i64 {
    let mut result: i64 = 0;
    let mut vendors: Vec<i32> = vec![];
    for l in lines {
        vendors.push(l.parse::<i32>().unwrap());
    }
    let perm = 2000;
    for mut v in vendors {
        for _ in 0..perm {
            v = ((v << 6) ^ v) & 16777215;
            v = ((v >> 5) ^ v) & 16777215;
            v = ((v << 11) ^ v) & 16777215;
        }
        result += v as i64;
    }

    result
}

fn solve2(lines: &Vec<String>) -> i64 {
    let mut secret_numbers: Vec<i64> = vec![];
    let mut gain: HashMap<Vec<i32>, i32> = HashMap::new();
    let mut seen: HashSet<Vec<i32>> = HashSet::new();
    let mut sequence: Vec<i32> = vec![];
    let mut last: i32 = 0;
    let mut price: i32;
    let mut change: i32;
    for l in lines {
        secret_numbers.push(l.parse::<i64>().unwrap());
    }
    for mut secret in secret_numbers {
        seen.clear();
        for _ in 0..2000 {
            secret = ((secret << 6) ^ secret) & 16777215;
            secret = ((secret >> 5) ^ secret) & 16777215;
            secret = ((secret << 11) ^ secret) & 16777215;
            price = (secret % 10) as i32;
            change = price - last;
            last = price;
            sequence.push(change);
            if sequence.len() == 5 {
                sequence.remove(0);
            }
            if sequence.len() == 4 {
                if !seen.contains(&sequence) {
                    *gain.entry(sequence.clone()).or_insert(0) += price;
                    seen.insert(sequence.clone());
                }
            }
        }
    }
    for g in &gain {
        if *g.1 == 2047 {
            println!("{:?}", g.0);
        }
    }
    *gain.values().max().unwrap() as i64
}
