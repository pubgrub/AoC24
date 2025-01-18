use file;
use indexmap::IndexSet;
use std::collections::HashMap;

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
    //println!("{:?}", pattern);

    for l in &lines[2..lines.len()] {
        println!("{}", l);
        let mut pat_1: Vec<String> = vec![];
        for p in &pattern {
            if l.contains(p) {
                pat_1.push(p.to_string());
            }
        }
        //println!("usable pattern: {}\n{:?}", pat.len(), pat);
        let mut pat_3: Vec<String> = vec![];
        let mut pat_3_max_length = 0;

        for i in &pat_1 {
            for j in &pat_1 {
                if l.contains(&(i.clone() + j.as_str())) {
                    for k in &pat_1 {
                        let lpat = i.clone() + j.as_str() + k.as_str();
                        if l.contains(&lpat) {
                            pat_3.push(lpat.clone());
                            pat_3_max_length = pat_3_max_length.max(lpat.len());
                        }
                    }
                }
            }
        }
        //println!("len: {}\npat: {:?}", long_pattern.len(), long_pattern);
    }
    //     let mut start_j = 0;
    //     while start_j < pat.len() {
    //         let old_len = pat.len();
    //         for i in 0..old_len {
    //             for j in start_j..old_len {
    //                 let comb_str =
    //                     pat.get_index(i).unwrap().to_string() + pat.get_index(j).unwrap();
    //                 if l.contains(&comb_str) {
    //                     if !pat.contains(&comb_str) {
    //                         // println!(
    //                         //     "new pat: {} + {} -> {}",
    //                         //     pat.get_index(i).unwrap(),
    //                         //     pat.get_index(j).unwrap(),
    //                         //     comb_str
    //                         // );
    //                         pat.insert(comb_str.clone());
    //                         pat_mult.push(pat_mult[i] * pat_mult[j]);
    //                     } else {
    //                         let pat_index = pat.get_index_of(comb_str.as_str()).unwrap();
    //                         if i != j {
    //                             // println!(
    //                             //     "old pat: {} + {} -> {} * {} = {}",
    //                             //     pat.get_index(i).unwrap(),
    //                             //     pat.get_index(j).unwrap(),
    //                             //     pat_mult[i],
    //                             //     pat_mult[j],
    //                             //     pat_mult[pat_index]
    //                             // );
    //                             pat_mult[pat_index] += pat_mult[i] * pat_mult[j];
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         for i in 0..pat.len() {
    //             print!("{:?}: {} ", pat.get_index(i).unwrap(), pat_mult[i]);
    //         }
    //         println!();
    //         start_j = old_len + 1;
    //     }
    //     println!(
    //         "pattern: {}, combined pattern: {}",
    //         pattern.len(),
    //         pat.len()
    //     );
    //     if pat.contains(l) {
    //         println!(
    //             "solution for {}: {}",
    //             l,
    //             pat_mult[pat.get_index_of(l).unwrap() as usize]
    //         );
    //     } else {
    //         println!("no solution for : {}", l);
    //     }
    //     //result += s2(&pat, l, &"".to_string(), vec![]);
    //     //println!("result: {}", result);
    // }
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
