use std::collections::HashMap;

use debug::{self, pause};
use file;
use itertools::Itertools;

const TEST: i32 = 1;

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
            print_map(' ', &map, width);
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
                                    move_cursor = true;
                                    move_block = true;
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
                    print_map(c, &map, width);
                } else {
                    // up-down
                    let tiles_to_move: Vec<i32> = vec![];
                    let (do_move, mut ret_tiles_to_move) = walk(&map, tiles_to_move, cursor, off);
                    if do_move == true {
                        ret_tiles_to_move.reverse();
                        for t in &ret_tiles_to_move {
                            println!("t: {}, off: {}", *t, off);
                            println!(
                                "map[t]: {}, map[t + off]: {}",
                                map[(*t + off) as usize],
                                map[*t as usize]
                            );
                            map[(*t + off) as usize] = map[*t as usize];
                            map[*t as usize] = 3;
                            println!(
                                "map[t]: {}, map[t + off]: {}",
                                map[(*t + off) as usize],
                                map[*t as usize]
                            );
                        }
                    }
                    print_map(c, &map, width);
                }
            }
        }
    }
    print_map(' ', &map, width);
    for (i, c) in map.iter().enumerate() {
        if c == &0usize {
            result += i as i32 / width * 100 + i as i32 % width;
        }
    }

    fn walk(map: &Vec<usize>, mut to_move: Vec<i32>, cursor: i32, off: i32) -> (bool, Vec<i32>) {
        let mut is_valid: bool;
        let test_pos = cursor + off;
        if map[test_pos as usize] == 2 {
            return (false, vec![]);
        }
        if map[test_pos as usize] == 3 {
            to_move.push(cursor);
            return (true, to_move);
        }
        let mut next_pos: Vec<i32> = vec![test_pos];
        if cursor == 0 {
            next_pos.push(cursor + off + 1)
        } else {
            next_pos.push(cursor + off - 1);
        }
        let mut ret_to_move: Vec<i32>;
        for p in &next_pos {
            (is_valid, ret_to_move) = walk(map, to_move, *p, off);
            if is_valid == false {
                return (false, vec![]);
            }
            to_move = ret_to_move;
        }
        (true, to_move)
    }

    result
}

fn print_map(dir: char, map: &Vec<usize>, width: i32) {
    let sym = vec!['[', ']', '#', '.', '@'];
    println!("dir: {}", dir);
    for (i, m) in map.iter().enumerate() {
        print!("{}", sym[*m]);
        if i % width as usize == width as usize - 1 {
            println!("");
        }
    }
    println!("");
    pause();
}
