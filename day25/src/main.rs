use file;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let mut lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&mut lines));
}

fn solve1(lines: &mut Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut data_type: i32 = 0;
    let mut keys: Vec<u64> = vec![];
    let mut locks: Vec<u64> = vec![];
    let mut line: u32 = 0;
    let mut val: u64 = 0;
    lines.push("".to_string());
    for l in lines {
        if l.len() == 0 {
            if data_type == 1 {
                keys.push(val);
            } else {
                locks.push(val);
            }
            line = 0;
            val = 0;
            data_type = 0;
            continue;
        }
        if data_type == 0 {
            if &l[0..1] == "." {
                data_type = 1 // key
            } else {
                data_type = 2 // lock
            }
        }
        for (i, c) in l.chars().enumerate() {
            if c == '#' {
                val += 2u64.pow(line * 5 + i as u32);
            }
        }
        line += 1;
    }

    for k in keys {
        for l in &locks {
            if k & l == 0 {
                result += 1;
            }
        }
    }

    result
}
