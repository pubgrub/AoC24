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
    let mut goal_str;
    let mut vals_str;
    let mut goal: i64;
    let mut vals: Vec<i64>;
    for line in lines {
        (goal_str, vals_str) = line.split_once(": ").unwrap();
        goal = goal_str.parse().unwrap();
        vals = vals_str.split(" ").map(|s| s.parse().unwrap()).collect();
        if s1(&vals, 1, vals[0], goal) == goal {
            result += goal;
        }
    }
    result
}

fn solve2(lines: &Vec<String>) -> i64 {
    let mut result: i64 = 0;
    let mut goal_str;
    let mut vals_str;
    let mut goal: i64;
    let mut vals: Vec<i64>;
    for line in lines {
        (goal_str, vals_str) = line.split_once(": ").unwrap();
        goal = goal_str.parse().unwrap();
        vals = vals_str.split(" ").map(|s| s.parse().unwrap()).collect();
        if s2(&vals, 1, vals[0], goal) == goal {
            result += goal;
        }
    }
    result
}

fn s1(vals: &Vec<i64>, i: usize, res: i64, goal: i64) -> i64 {
    if res > goal || i == vals.len() {
        return res;
    }
    if s1(vals, i + 1, res + vals[i], goal) == goal {
        return goal;
    };
    if s1(vals, i + 1, res * vals[i], goal) == goal {
        return goal;
    };
    0
}

fn s2(vals: &Vec<i64>, i: usize, res: i64, goal: i64) -> i64 {
    if res > goal || i == vals.len() {
        return res;
    }
    if s2(vals, i + 1, res + vals[i], goal) == goal {
        return goal;
    };
    if s2(vals, i + 1, res * vals[i], goal) == goal {
        return goal;
    };
    if s2(
        vals,
        i + 1,
        res * usize::pow(10, vals[i].to_string().len() as u32) as i64 + vals[i],
        goal,
    ) == goal
    {
        return goal;
    }
    0
}
