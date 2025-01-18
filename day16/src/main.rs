use std::collections::{HashMap, HashSet};
use std::i32::MAX;

use file;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut grid: Vec<i32> = vec![];
    let width: i32 = lines[0].len() as i32;
    for l in lines {
        grid.append(
            &mut l
                .chars()
                .map(|c| match c {
                    '#' => 1,
                    'S' => 2,
                    'E' => 3,
                    _ => 0,
                })
                .collect(),
        )
    }
    let start: i32 = grid.iter().position(|&x| x == 2).unwrap() as i32;
    let end: i32 = grid.iter().position(|&x| x == 3).unwrap() as i32;
    grid[start as usize] = 0;
    grid[end as usize] = 0;

    let offs: Vec<i32> = vec![-width, 1, width, -1];
    let mut cost: HashMap<(i32, i32), i32> = HashMap::new(); // (pos,dir),cost
    let mut to_do: HashSet<(i32, i32)> = HashSet::new(); // (pos,dir)
    to_do.insert((start, 1));
    cost.insert((start, 1), 0);

    let turn_cost = 1000;
    let move_cost = 1;
    let mut akt_pos;
    let mut akt_dir;
    let mut next_pos;
    let mut next_cost: i32;
    while to_do.len() > 0 {
        (akt_pos, akt_dir) = to_do.iter().next().unwrap().clone();
        let akt_cost = *cost.get(&(akt_pos, akt_dir)).unwrap();
        // turn right
        next_pos = (akt_pos, (akt_dir + 1) % 4);
        next_cost = akt_cost + turn_cost;
        if !cost.contains_key(&next_pos) || cost.get(&next_pos).unwrap() > &next_cost {
            cost.insert(next_pos, next_cost);
            to_do.insert(next_pos);
        }
        // turn left
        next_pos = (akt_pos, (akt_dir - 1).rem_euclid(4));
        next_cost = akt_cost + turn_cost;
        if !cost.contains_key(&next_pos) || cost.get(&next_pos).unwrap() > &next_cost {
            cost.insert(next_pos, next_cost);
            to_do.insert(next_pos);
        }
        // move straight
        let move_pos = akt_pos + offs[akt_dir as usize];
        if grid[move_pos as usize] == 0 {
            next_pos = (akt_pos + offs[akt_dir as usize], akt_dir);
            next_cost = akt_cost + move_cost;
            if !cost.contains_key(&next_pos) || cost.get(&next_pos).unwrap() > &next_cost {
                cost.insert(next_pos, next_cost);
                to_do.insert(next_pos);
            }
        }
        to_do.remove(&(akt_pos, akt_dir));
    }

    let mut total_cost = MAX as i32;
    for d in 0..=3 {
        if cost.contains_key(&(end, d)) {
            total_cost = total_cost.min(*cost.get(&(end, d)).unwrap());
        }
    }

    total_cost
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut grid: Vec<i32> = vec![];
    let width: i32 = lines[0].len() as i32;
    for l in lines {
        grid.append(
            &mut l
                .chars()
                .map(|c| match c {
                    '#' => 1,
                    'S' => 2,
                    'E' => 3,
                    _ => 0,
                })
                .collect(),
        )
    }
    let start: i32 = grid.iter().position(|&x| x == 2).unwrap() as i32;
    let end: i32 = grid.iter().position(|&x| x == 3).unwrap() as i32;
    grid[start as usize] = 0;
    grid[end as usize] = 0;

    let offs: Vec<i32> = vec![-width, 1, width, -1];
    let mut cost: HashMap<(i32, i32), i32> = HashMap::new(); // (pos,dir),cost
    let mut to_do: HashSet<(i32, i32)> = HashSet::new(); // (pos,dir)
    to_do.insert((start, 1));
    cost.insert((start, 1), 0);

    let turn_cost = 1000;
    let move_cost = 1;
    let mut akt_pos;
    let mut akt_dir;
    let mut next_pos;
    let mut next_cost: i32;
    let mut akt_cost;
    while !to_do.is_empty() {
        (akt_pos, akt_dir) = to_do.iter().next().unwrap().clone();
        akt_cost = *cost.get(&(akt_pos, akt_dir)).unwrap();
        // turn right
        next_pos = (akt_pos, (akt_dir + 1) % 4);
        next_cost = akt_cost + turn_cost;
        if !cost.contains_key(&next_pos) || cost.get(&next_pos).unwrap() > &next_cost {
            cost.insert(next_pos, next_cost);
            to_do.insert(next_pos);
        }
        // turn left
        next_pos = (akt_pos, (akt_dir + 3) % 4);
        next_cost = akt_cost + turn_cost;
        if !cost.contains_key(&next_pos) || cost.get(&next_pos).unwrap() > &next_cost {
            cost.insert(next_pos, next_cost);
            to_do.insert(next_pos);
        }
        // move straight
        let move_pos = akt_pos + offs[akt_dir as usize];
        if grid[move_pos as usize] == 0 {
            next_pos = (akt_pos + offs[akt_dir as usize], akt_dir);
            next_cost = akt_cost + move_cost;
            if !cost.contains_key(&next_pos) || cost.get(&next_pos).unwrap() > &next_cost {
                cost.insert(next_pos, next_cost);
                to_do.insert(next_pos);
            }
        }
        to_do.remove(&(akt_pos, akt_dir));
    }

    let mut total_cost = MAX as i32;
    let mut valid_dirs: Vec<i32> = vec![];
    for d in 0..=3 {
        if cost.contains_key(&(end, d)) {
            total_cost = total_cost.min(*cost.get(&(end, d)).unwrap());
            valid_dirs.push(d);
        }
    }

    let mut seats: HashSet<i32> = HashSet::new();
    seats.insert(end);
    to_do.clear();
    for d in valid_dirs {
        if *cost.get(&(end, d)).unwrap() == total_cost {
            to_do.insert((end, d));
        }
    }
    let mut next_dir;
    while !to_do.is_empty() {
        (akt_pos, akt_dir) = to_do.iter().next().unwrap().clone();
        akt_cost = *cost.get(&(akt_pos, akt_dir)).unwrap();
        // turn left
        next_dir = (akt_dir + 1) % 4;
        next_pos = (akt_pos, next_dir);
        if *cost.get(&next_pos).unwrap() + turn_cost == akt_cost {
            to_do.insert((akt_pos, next_dir));
        }
        // turn right
        next_dir = (akt_dir + 3) % 4;
        next_pos = (akt_pos, next_dir);
        if *cost.get(&next_pos).unwrap() + turn_cost == akt_cost {
            to_do.insert((akt_pos, next_dir));
        }
        // move back
        let move_pos = akt_pos - offs[akt_dir as usize];
        next_pos = (move_pos, akt_dir);
        if grid[move_pos as usize] == 0 {
            if *cost.get(&next_pos).unwrap() + move_cost == akt_cost {
                seats.insert(move_pos);
                to_do.insert((move_pos, akt_dir));
            }
        }
        to_do.remove(&(akt_pos, akt_dir));
    }

    seats.len() as i32
}
