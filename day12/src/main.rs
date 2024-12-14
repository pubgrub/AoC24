use debug;
use file;
use std::collections::HashSet;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let width = lines[0].len() + 2;
    let mut grid: String = ".".repeat(width);
    for l in lines {
        grid.push('.');
        grid.push_str(l);
        grid.push('.');
    }
    grid.push_str(&".".repeat(width));
    let offs = vec![-(width as i32), 1, width as i32, -1];
    for i in width + 1..grid.len() - width - 1 {
        let mut todo: HashSet<usize> = HashSet::new();
        let c = String::from(&grid[i..i + 1]);
        let c_lower = c.to_lowercase();
        if c == "." || c.chars().next().unwrap().is_lowercase() {
            continue;
        }
        todo.insert(i);
        let mut area = 0;
        let mut fence = 0;
        let mut test_pos;
        while todo.len() > 0 {
            //            println!("{}", todo.len());
            let akt_pos = todo.iter().next().unwrap().clone();
            todo.remove(&akt_pos);
            area += 1;
            for off in &offs {
                test_pos = (akt_pos as i32 + off) as usize;
                let test_field = &grid[test_pos..test_pos + 1];
                if test_field == "." {
                    fence += 1;
                    continue;
                }
                if test_field == c {
                    todo.insert(test_pos);
                    continue;
                }
                if test_field != c_lower {
                    fence += 1;
                }
            }
            grid.replace_range(akt_pos..akt_pos + 1, &c_lower);
        }
        result += area * fence;
        //println!("{} {} {}", c, area, fence);
    }

    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    result
}
