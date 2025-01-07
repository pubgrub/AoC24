use std::collections::HashMap;
use std::usize::MAX;

use file;
use itertools::Itertools;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut grid: Vec<char>;
    let mut cost: HashMap<usize, i32> = HashMap::new();
    let width: i32 = (lines[0].len() + 2) as i32;
    let offs: Vec<i32> = vec![-width, 1, width, -1];

    grid = vec!['#'; width as usize];

    for l in lines {
        grid.push('#');
        grid.append(&mut l.chars().collect());
        grid.push('#');
    }
    grid.append(&mut vec!['#'; width as usize]);
    let start;
    let end;
    (start, _) = grid.iter().find_position(|&&c| c == 'S').unwrap();
    (end, _) = grid.iter().find_position(|&&c| c == 'E').unwrap();
    let mut last_pos: usize = MAX;
    cost.insert(start, 0);

    let mut akt_pos = start;
    let mut next_pos;
    let mut akt_cost = 0;
    while akt_pos != end {
        for d in 0..=3 {
            next_pos = (akt_pos as i32 + offs[d]) as usize;
            if next_pos == last_pos || grid[next_pos] == '#' {
                continue;
            }
            last_pos = akt_pos;
            akt_pos = next_pos;
            akt_cost += 1;
            cost.insert(akt_pos, akt_cost);
            break;
        }
    }

    for (p, g) in grid.iter().enumerate() {
        if g != &'#' {
            for d in 0..=3 {
                if cost.contains_key(&((p as i32 + offs[d] * 2) as usize)) {
                    let diff = cost.get(&p).unwrap()
                        - cost.get(&((p as i32 + offs[d] * 2) as usize)).unwrap()
                        - 2;
                    if diff >= 100 {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut grid: Vec<char>;
    let mut cost: HashMap<usize, i32> = HashMap::new();
    let width: i32 = (lines[0].len() + 40) as i32;
    let offs: Vec<i32> = vec![-width, 1, width, -1];

    grid = vec!['#'; width as usize];
    let border = vec!['#'; 20];
    for l in lines {
        grid.append(&mut border.clone());
        grid.append(&mut l.chars().collect());
        grid.append(&mut border.clone());
    }
    grid.append(&mut vec!['#'; width as usize]);
    let start;
    let end;
    (start, _) = grid.iter().find_position(|&&c| c == 'S').unwrap();
    (end, _) = grid.iter().find_position(|&&c| c == 'E').unwrap();
    let mut last_pos: usize = MAX;
    cost.insert(start, 0);

    let mut akt_pos = start;
    let mut next_pos;
    let mut akt_cost = 0;
    while akt_pos != end {
        for d in 0..=3 {
            next_pos = (akt_pos as i32 + offs[d]) as usize;
            if next_pos == last_pos || grid[next_pos] == '#' {
                continue;
            }
            last_pos = akt_pos;
            akt_pos = next_pos;
            akt_cost += 1;
            cost.insert(akt_pos, akt_cost);
            break;
        }
    }

    for (p, g) in grid.iter().enumerate() {
        if g != &'#' {
            for y in -20i32..=20i32 {
                let y_abs = y.abs();
                let dx = 20 - y_abs;
                for x in -dx..=dx {
                    let x_abs = x.abs();
                    let offset = y * width + x;
                    if cost.contains_key(&((p as i32 + offset) as usize)) {
                        let diff = cost.get(&p).unwrap()
                            - cost.get(&((p as i32 + offset) as usize)).unwrap()
                            - x_abs
                            - y_abs;
                        if diff >= 100 {
                            result += 1;
                        }
                    }
                }
            }
        }
    }

    result
}
