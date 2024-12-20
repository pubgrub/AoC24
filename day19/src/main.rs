use file;
use std::{collections::HashMap, ops::Index};
use indexmap::IndexSet;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let pattern: Vec<&str> = lines[0].split(", ").collect();
    println!("{:?}", pattern);

    for l in &lines[2..lines.len()] {
        //println!("{}", l);
        let possible: Vec<String> = vec![];
        let res;
        (res, _) = s1(&pattern, l, &"".to_string(), possible);
        if res == true {
            result += 1;
            //println!("result: {}", result);
        }
    }

    result
}

fn s1(
    pattern: &Vec<&str>,
    target: &String,
    build: &String,
    mut possible: Vec<String>,
) -> (bool, Vec<String>) {
    //println!("b: {},{},{}", build, build.len(), target.len());
    //println!("p: {:?}", possible);
    if target == build {
        return (true, possible);
    }
    if build.len() >= target.len() || possible.contains(build) {
        //println!("possible known: {} in {:?}", build, possible);
        return (false, possible);
    }
    //println!("inserting into possible: {},{}", build, possible.len());
    possible.push(build.clone());
    //println!("inserted: {},{}", build, possible.len());

    for p in pattern {
        let test_str = build.clone() + *p;
        //println!("testing: {} + {} -> {}", build, p, test_str);
        if target.starts_with(test_str.as_str()) {
            //println!("t: {}", test_str);
            //            new_possible.push(value);
            //let returned_possible: Vec<String>;
            let (res, returned_possible) = s1(pattern, target, &test_str, possible);
            possible = returned_possible;
            if res == true {
                return (true, possible);
            }
        }
    }
    (false, possible)
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let pattern: Vec<&str> = lines[0].split(", ").collect();
    println!("{:?}", pattern);

    for l in &lines[2..lines.len()] {
        println!("{}", l);
        let mut pat: Vec<&str> = vec![];
        let mut pat_mult: Vec<i32> = vec![];
        for p in &pattern {
            if l.contains(p) {
                pat.push(p);
                pat_mult.push(1);
            }
        }
        for i in 0..pat.len() {
            for j in 0..pat.len() {
                let comb_str = pat[i].to_string() + pat[j];
                if l.contains(comb_str.as_str()) {
                    if ! pat.contains(comb_str) {
                        pat.push(&comb_str);
                        pat_mult.push(pat_mult[i] * pat_mult[j]);
                    else {
                        pat_mult[pat.  ]
                    }


                        if pat
                }
            }
        }
        println!("pattern: {}, local: {}", pattern.len(), pat.len());

        result += s2(&pat, l, &"".to_string(), vec![]);
        //println!("result: {}", result);
    }
    result
}

fn s2(pattern: &Vec<&str>, target: &String, build: &String, pieces: Vec<i32>) -> i32 {
    //println!("b: {},{},{}", build, build.len(), target.len());
    let mut poss: i32 = 0;
    if build.len() > target.len() {
        return 0;
    }
    if target == build {
        println!("{:?}", pieces);
        return 1;
    }

    //println!("not found: {}", build);

    for (i, p) in pattern.iter().enumerate() {
        let mut test_str = build.clone();
        test_str += *p;
        //println!("testing: {} + {} -> {}", build, p, test_str);
        if target.starts_with(test_str.as_str()) {
            //println!("t: {}", test_str);
            let mut new_pieces = pieces.clone();
            new_pieces.push(i as i32);
            poss += s2(pattern, target, &test_str, new_pieces);
        }
    }
    poss
}
