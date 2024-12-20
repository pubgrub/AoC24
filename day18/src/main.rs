use file;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut x: i32;
    let mut y: i32;
    let width: i32;
    let height;
    let n_blocks: usize;
    if TEST == 0 {
        width = 73;
        height = 73;
        n_blocks = 1024;
    } else {
        width = 9;
        height = 9;
        n_blocks = 12;
    }
    let mut blocked: HashSet<i32> = HashSet::new();
    for l in &lines[0..n_blocks] {
        (x, y) = l
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        blocked.insert((y + 1) * width + x + 1);
    }
    for i in 0..width {
        blocked.insert(i);
        blocked.insert(width * (height - 1) + i);
    }
    for i in 1..height {
        blocked.insert(i * width);
        blocked.insert((i + 1) * width - 1);
    }

    let offs: Vec<i32> = vec![-width, 1, width, -1];
    let mut grid: HashMap<i32, i32> = HashMap::new();
    grid.insert(width + 1, 0);

    let mut todo: HashSet<i32> = HashSet::new();
    todo.insert(width + 1);

    while todo.len() > 0 {
        let akt_pos = todo.iter().next().unwrap().clone();
        todo.remove(&akt_pos);
        let akt_cost = grid.get(&akt_pos).unwrap().clone();
        for off in &offs {
            let next_pos = akt_pos + off;
            if blocked.contains(&next_pos) {
                continue;
            }
            if !grid.contains_key(&next_pos) {
                grid.insert(next_pos, akt_cost + 1);
                todo.insert(next_pos);
            } else {
                if grid.get(&next_pos).unwrap() > &(akt_cost + 1) {
                    grid.insert(next_pos, akt_cost + 1);
                    todo.insert(next_pos);
                }
            }
        }
    }
    return *grid.get(&(width * (height - 1) - 2)).unwrap();
}

fn solve2(lines: &Vec<String>) -> String {
    let mut x: i32;
    let mut y: i32;
    let width: i32;
    let height;
    let n_blocks: usize;
    if TEST == 0 {
        width = 73;
        height = 73;
        n_blocks = 1024;
    } else {
        width = 9;
        height = 9;
        n_blocks = 12;
    }
    let mut blocked: HashSet<i32> = HashSet::new();
    let mut blocks: Vec<i32> = vec![];

    for l in lines {
        (x, y) = l
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        blocks.push((y + 1) * width + x + 1);
    }
    for b in blocks[0..n_blocks].iter() {
        blocked.insert(*b);
    }
    for i in 0..width {
        blocked.insert(i);
        blocked.insert(width * (height - 1) + i);
    }
    for i in 1..height {
        blocked.insert(i * width);
        blocked.insert((i + 1) * width - 1);
    }
    let offs: Vec<i32> = vec![-width, 1, width, -1];
    let init_trail = vec![width + 1];
    let mut trail;
    let goal = width * (height - 1) - 2;

    (_, trail) = walk2(&blocked, init_trail.clone(), goal, &offs);
    for block in &blocks[n_blocks..blocks.len()] {
        let next_block = *block;
        blocked.insert(next_block);
        if !trail.clone().contains(&next_block) {
            continue;
        }
        let res: bool;
        (res, trail) = walk2(&blocked.clone(), init_trail.clone(), goal, &offs);
        if res == false {
            let y = next_block / width - 1;
            let x = next_block - (y + 1) * width - 1;
            return x.to_string() + "," + &y.to_string();
        }
    }
    "not valid".to_string()
}

fn walk2(blocked: &HashSet<i32>, trail: Vec<i32>, goal: i32, offs: &Vec<i32>) -> (bool, Vec<i32>) {
    let akt_pos = trail.last().unwrap().clone();
    if akt_pos == goal {
        return (true, trail);
    }
    for off in offs {
        let next_pos = akt_pos + off;
        if blocked.contains(&next_pos) || trail.contains(&next_pos) {
            continue;
        }
        let mut new_trail = trail.clone();
        new_trail.push(next_pos);
        let res;
        let res_trail;
        (res, res_trail) = walk2(blocked, new_trail, goal, offs);
        if res == true {
            return (true, res_trail);
        }
    }
    (false, vec![])
}
