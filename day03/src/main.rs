use file;
use regex::Regex;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let re = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    for line in lines {}

    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    result
}
