use std::collections::{HashMap, HashSet};

use file;
use itertools::Itertools;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut groups: HashMap<String, HashSet<String>> = HashMap::new();
    let mut pairs: Vec<HashSet<String>> = vec![];
    let mut elem: HashSet<String> = HashSet::new();
    let mut triple: Vec<HashSet<String>> = vec![];
    for l in lines {
        let left;
        let right;
        (left, right) = l.split("-").map(|s| s.to_string()).collect_tuple().unwrap();
        pairs.push(HashSet::from([left.clone(), right.clone()]));
        elem.insert(left.clone());
        elem.insert(right.clone());
    }
    println!("fertig mit lesen...");
    println!("elem: {}", elem.len());
    for pair in &pairs {
        println!("pair: {:?}", pair);
        for el in &elem {
            if pair.contains(el) {
                continue;
            }
            let mut match_count = 0;
            for p in pair {
                if pairs.contains(&HashSet::from([p.clone(), el.clone()])) {
                    match_count += 1;
                }
            }
            if match_count == 2 {
                let mut new_set = pair.clone();
                new_set.insert(el.clone());

                if !triple.contains(&new_set) {
                    triple.push(new_set);
                }
            }
        }
    }
    println!("triples generated...");
    for t in triple {
        let mut found = false;
        for s in &t {
            if &s[0..1] == "t" {
                found = true;
                break;
            }
        }
        if found == true {
            result += 1;
        }
    }

    println!("{:?}", pairs);
    println!("{:?}", elem);
    //    println!("{:?}", triple);
    for g in groups {
        if g.1.len() == 3 {
            println!("{:?}", g.1);
        }
    }
    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    result
}
