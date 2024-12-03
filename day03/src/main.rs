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
    let re1 = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let re2 = Regex::new(r"(\d+)").unwrap();
    for line in lines {
        let muls: Vec<&str> = re1.find_iter(line).map(|s| s.as_str()).collect();
        for mul in muls {
            let factors: Vec<&str> = re2.find_iter(mul).map(|s| s.as_str()).collect();
            result += factors[0].parse::<i32>().unwrap() * factors[1].parse::<i32>().unwrap();
        }
    }
    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut enabled = true;
    let re1 = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
    let re2 = Regex::new(r"(\d+)").unwrap();
    for line in lines {
        let instructions: Vec<&str> = re1.find_iter(line).map(|s| s.as_str()).collect();
        for instr in instructions {
            match instr {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        let factors: Vec<&str> = re2.find_iter(instr).map(|s| s.as_str()).collect();
                        result +=
                            factors[0].parse::<i32>().unwrap() * factors[1].parse::<i32>().unwrap();
                    }
                }
            }
        }
    }
    result
}
