use file;
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
    let max_x = lines[0].len() as i32 - 1;
    let max_y = lines.len() as i32 - 1;
    let mut radar_coords: HashMap<char, Vec<(i32, i32)>> = Default::default();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                radar_coords
                    .entry(c)
                    .or_insert(vec![])
                    .push((x as i32, y as i32));
            }
        }
    }
    let mut x_offs: i32;
    let mut y_offs: i32;
    let mut anti_x: i32;
    let mut anti_y: i32;

    for (_c, coords) in radar_coords.iter() {
        for (i, (x1, y1)) in coords[0..coords.len() - 1].iter().enumerate() {
            for (x2, y2) in coords[i + 1..coords.len()].iter() {
                x_offs = x1 - x2;
                y_offs = y1 - y2;
                anti_x = x1 + x_offs;
                anti_y = y1 + y_offs;
                if anti_x >= 0 && anti_x <= max_x && anti_y >= 0 && anti_y <= max_y {
                    antinodes.insert((anti_x, anti_y));
                }
                anti_x = x2 - x_offs;
                anti_y = y2 - y_offs;
                if anti_x >= 0 && anti_x <= max_x && anti_y >= 0 && anti_y <= max_y {
                    antinodes.insert((anti_x, anti_y));
                }
            }
        }
    }
    antinodes.len() as i32
}

fn solve2(lines: &Vec<String>) -> i32 {
    let max_x = lines[0].len() as i32 - 1;
    let max_y = lines.len() as i32 - 1;
    let mut radar_coords: HashMap<char, Vec<(i32, i32)>> = Default::default();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                radar_coords
                    .entry(c)
                    .or_insert(vec![])
                    .push((x as i32, y as i32));
            }
        }
    }
    let mut x_offs: i32;
    let mut y_offs: i32;
    let mut anti_x: i32;
    let mut anti_y: i32;

    for (_c, coords) in radar_coords.iter() {
        for (i, (x1, y1)) in coords[0..coords.len() - 1].iter().enumerate() {
            for (x2, y2) in coords[i + 1..coords.len()].iter() {
                antinodes.insert((*x1, *y1));
                antinodes.insert((*x2, *y2));
                x_offs = x1 - x2;
                y_offs = y1 - y2;
                anti_x = *x1;
                anti_y = *y1;
                loop {
                    anti_x += x_offs;
                    anti_y += y_offs;
                    if anti_x >= 0 && anti_x <= max_x && anti_y >= 0 && anti_y <= max_y {
                        antinodes.insert((anti_x, anti_y));
                    } else {
                        break;
                    }
                }
                anti_x = *x2;
                anti_y = *y2;
                loop {
                    anti_x = anti_x - x_offs;
                    anti_y = anti_y - y_offs;
                    if anti_x >= 0 && anti_x <= max_x && anti_y >= 0 && anti_y <= max_y {
                        antinodes.insert((anti_x, anti_y));
                    } else {
                        break;
                    }
                }
            }
        }
    }
    antinodes.len() as i32
}
