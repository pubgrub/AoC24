use file;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn chk_line(levels: &Vec<i32>) -> bool {
    if levels[0] == levels[1] {
        return false;
    };
    let climbing = levels[0] < levels[1];
    for (i, lvl) in levels[..levels.len() - 1].iter().enumerate() {
        let diff = levels[i + 1] - lvl;
        if diff.abs() == 0 || diff.abs() > 3 || ((diff > 0) != climbing) {
            return false;
        };
    }
    true
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    for line in lines {
        let levels: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
        if chk_line(&levels) {
            result += 1;
        }
    }
    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    'lvls: for line in lines {
        let levels: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
        if chk_line(&levels) {
            result += 1;
            continue;
        }
        for i in 0..levels.len() {
            let mut short_levels = levels.clone();
            short_levels.remove(i);
            if chk_line(&short_levels) {
                result += 1;
                continue 'lvls;
            }
        }
    }

    result
}
