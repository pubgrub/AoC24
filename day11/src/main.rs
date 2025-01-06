use debug;
use file;
use std::collections::HashMap;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve(&lines, 25));
    println!("Part 2: {}", solve(&lines, 75));
}

fn solve(lines: &Vec<String>, max_iter: usize) -> i64 {
    let mut result: i64 = 0;
    let numbers: Vec<i64> = lines[0]
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut cache: HashMap<(i64, usize), i64> = HashMap::new();
    let mut r;
    for n in numbers {
        (r, cache) = run(n, 0, max_iter, cache);
        result += r as i64;
    }

    fn run(
        i: i64,
        lvl: usize,
        max_iter: usize,
        mut cache: HashMap<(i64, usize), i64>,
    ) -> (i64, HashMap<(i64, usize), i64>) {
        let mut count: i64 = 0;
        let mut to_test: Vec<i64> = vec![];
        let mut r;
        if lvl == max_iter {
            return (1, cache);
        }
        if i == 0 {
            to_test.push(1);
        } else {
            let digits: u32 = i.to_string().chars().count() as u32;
            if digits % 2 == 0 {
                let exp = 10i64.pow(digits / 2);
                to_test.push(i / exp);
                to_test.push(i % exp);
            } else {
                to_test.push(i * 2024);
            }
        }
        for item in to_test {
            if cache.contains_key(&(item, lvl)) {
                count += *cache.get(&(item, lvl)).unwrap() as i64;
            } else {
                (r, cache) = run(item, lvl + 1, max_iter, cache);
                cache.insert((item, lvl), r);
                count += r as i64;
            }
        }
        (count, cache)
    }
    result
}
