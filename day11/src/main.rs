use file;
use std::{collections::HashMap, hash::Hash};

const TEST:i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}",solve1(&lines));
    println!("Part 2: {}",solve2(&lines));
}

fn solve1(lines:&Vec<String>) -> i32 {
    let mut result:i32 = 0;
    let start_entries:Vec<usize> = lines[0].split(" ").map(|x| (x.parse::<usize>() ).unwrap()).collect();
    let mut count_5:HashMap<usize,Vec<usize>> = HashMap::new();
    let mut list_5:HashMap<usize,Vec<usize>> = HashMap::new();
    let mut to_do = start_entries.clone();
    while to_do.len() > 0{
        if list_5.contains_key(&to_do[0]) {
            to_do.remove(0);
            continue;
        }
        let mut list:Vec<usize> = vec![to_do[0]];
        for i in 0..5 {
            let res_list:Vec<usize> = vec![];
            for item in &list {

            }
        }
    }
    
    result
}



fn solve2(lines:&Vec<String>) -> i32 {
    let mut result:i32 = 0;
    result
}
