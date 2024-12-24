use file;
use itertools::Itertools;
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
    let re1 = Regex::new(r"(\d+)").unwrap();

    for i in (0..lines.len()).step_by(4) {
        let xa: i32;
        let ya: i32;
        let xb: i32;
        let yb: i32;
        let xt: i32;
        let yt: i32;
        let a: i32;
        let b: i32;
        (xa, ya) = re1
            .find_iter(&lines[i])
            .map(|s| s.as_str().parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        (xb, yb) = re1
            .find_iter(&lines[i + 1])
            .map(|s| s.as_str().parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        (xt, yt) = re1
            .find_iter(&lines[i + 2])
            .map(|s| s.as_str().parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        if (yt * xa - xt * ya) % (yb * xa - xb * ya) == 0
            && (xt - (yt * xa - xt * ya) / (yb * xa - xb * ya) * xb) % xa == 0
        {
            b = (yt * xa - xt * ya) / (yb * xa - xb * ya);
            a = (xt - b * xb) / xa;
            result = result + a * 3 + b;
        }
    }

    result
}

fn solve2(lines: &Vec<String>) -> i64 {
    let mut result: i64 = 0;
    let re1 = Regex::new(r"(\d+)").unwrap();

    for i in (0..lines.len()).step_by(4) {
        let xa: i64;
        let ya: i64;
        let xb: i64;
        let yb: i64;
        let xt: i64;
        let yt: i64;
        let a: i64;
        let b: i64;
        (xa, ya) = re1
            .find_iter(&lines[i])
            .map(|s| s.as_str().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        (xb, yb) = re1
            .find_iter(&lines[i + 1])
            .map(|s| s.as_str().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        (xt, yt) = re1
            .find_iter(&lines[i + 2])
            .map(|s| s.as_str().parse::<i64>().unwrap() + 10000000000000)
            .collect_tuple()
            .unwrap();

        if (yt * xa - xt * ya) % (yb * xa - xb * ya) == 0
            && (xt - (yt * xa - xt * ya) / (yb * xa - xb * ya) * xb) % xa == 0
        {
            b = (yt * xa - xt * ya) / (yb * xa - xb * ya);
            a = (xt - b * xb) / xa;
            result = result + (a * 3 + b);
        }
    }

    result
}
