use std::collections::HashMap;

use debug::{self, pause};
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
    let width: i32 = lines[0].len() as i32;
    let offs: HashMap<char, i32> = HashMap::from([
        ('^', -(width as i32)),
        ('>', 1),
        ('v', width as i32),
        ('<', -1),
    ]);
    let legend: HashMap<char, usize> = HashMap::from([('O', 0), ('#', 1), ('.', 2), ('@', 3)]);
    let mut off: i32;

    let mut is_map = true;
    let mut map: Vec<usize> = vec![];
    let mut cursor: i32 = 0;
    let mut move_cursor;
    let mut next_cursor: i32 = 0;
    let mut move_block;
    let mut next_pos: i32;
    for l in lines {
        if l.len() == 0 {
            is_map = false;
            cursor = map.iter().position(|i| i == &3usize).unwrap() as i32;
            continue;
        }
        if is_map {
            map.append(&mut l.chars().map(|c| *legend.get(&c).unwrap()).collect_vec());
        } else {
            for c in l.chars() {
                move_cursor = false;
                move_block = false;
                off = *offs.get(&c).unwrap();
                next_pos = cursor;
                loop {
                    next_pos += off;
                    match map[next_pos as usize] {
                        0 => {
                            // O
                            if move_cursor == false {
                                move_block = true;
                                move_cursor = true;
                                next_cursor = next_pos;
                            }
                        }
                        1 => break, // #
                        2 => {
                            // .
                            if move_cursor == false {
                                next_cursor = next_pos;
                            }
                            map[cursor as usize] = 2;
                            cursor = next_cursor;
                            map[cursor as usize] = 3;
                            if move_block == true {
                                map[next_pos as usize] = 0;
                            }
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    for (i, c) in map.iter().enumerate() {
        if c == &0usize {
            result += i as i32 / width * 100 + i as i32 % width;
        }
    }
    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let width: i32 = lines[0].len() as i32 * 2;
    let offs: HashMap<char, i32> = HashMap::from([
        ('^', -(width as i32)),
        ('>', 1),
        ('v', width as i32),
        ('<', -1),
    ]);
    let legend: HashMap<char, (usize, usize)> =
        HashMap::from([('O', (0, 1)), ('#', (2, 2)), ('.', (3, 3)), ('@', (4, 3))]);
    let mut off: i32;

    let mut is_map = true;
    let mut map: Vec<usize> = vec![];
    let mut cursor: i32 = 0;
    let mut move_cursor;
    let mut next_cursor: i32 = 0;
    let mut move_block;
    let mut next_pos: i32;
    for l in lines {
        if l.len() == 0 {
            is_map = false;
            cursor = map.iter().position(|i| i == &4usize).unwrap() as i32;
            continue;
        }
        if is_map {
            for c in l.chars() {
                let (t1, t2) = legend.get(&c).unwrap();
                map.append(&mut vec![*t1, *t2]);
            }
        } else {
            for c in l.chars() {
                move_cursor = false;
                move_block = false;
                off = *offs.get(&c).unwrap();
                next_pos = cursor;
                if ['<', '>'].contains(&c) {
                    loop {
                        next_pos += off;
                        match map[next_pos as usize] {
                            0 | 1 => {
                                // O
                                if move_cursor == false {
                                    move_block = true;
                                    move_cursor = true;
                                    next_cursor = next_pos;
                                }
                            }
                            2 => break, // #
                            3 => {
                                // .
                                if move_cursor == false {
                                    next_cursor = next_pos;
                                }
                                if move_block == true {
                                    map.remove(next_pos as usize);
                                    map.insert(cursor as usize, 3);
                                } else {
                                    map[cursor as usize] = 3;
                                    cursor = next_cursor;
                                    map[cursor as usize] = 4;
                                }
                                break;
                            }
                            _ => {}
                        }
                    }
                } else {
                    // up-down
                    let move_rows: Vec<Vec<i32>> = vec![vec![cursor]];
                    let mut pushing_row = move_rows.len();
                    loop {
                        for i in move_rows[pushing_row].iter() {
                            let pushing_block = move_rows[pushing_row][*i as usize];
                        }

                        next_pos += off;
                    }
                }
            }
        }
    }
    for (i, c) in map.iter().enumerate() {
        if c == &0usize {
            result += i as i32 / width * 100 + i as i32 % width;
        }
    }

    fn walk() -> (bool, Vec<i32>) {
        (false, vec![])
    }
    result
}
