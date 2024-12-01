use file;
use std::collections::HashMap;

const TEST:i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    println!("{day}");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}",solve1(&lines));
    println!("Part 2: {}",solve2(&lines));
}

fn get_lists(lines:&Vec<String>) -> (Vec<i32>,Vec<i32>) {
    let mut list1:Vec<i32> = vec![];
    let mut list2:Vec<i32>= vec![];
    for line in lines {
        let nums:Vec<&str>= line.split("   ").collect();
        list1.push( nums[0].parse().unwrap());
        list2.push( nums[1].parse().unwrap());
    }
    (list1,list2)
}


fn solve1(lines:&Vec<String>) -> i32 {
    let mut list1:Vec<i32>;
    let mut list2:Vec<i32>;
    
    (list1,list2) = get_lists(lines);
    list1.sort();
    list2.sort();
    let mut diff = 0;
    for (i,x) in list1.iter().enumerate() {
        diff += ( x - list2[i]).abs(); 
    }
    diff
}

fn solve2(lines:&Vec<String>) -> i32 {        
    let list1:Vec<i32>;
    let list2:Vec<i32>;
    let mut count:HashMap<i32,i32 >  = HashMap::new();
    (list1,list2) = get_lists(lines);
    for i in list2 {
        *count.entry(i).or_insert(0) += 1;
    }
    let mut sum = 0;
    for i in list1 {
        sum += i * *count.entry(i).or_default();
    }
    sum    
}

