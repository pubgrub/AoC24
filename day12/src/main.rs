use file;
use std::collections::HashSet;
use std::collections::HashMap;

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
        let mut borders:Vec<HashMap<usize,Vec<(usize,usize)>>> = vec![HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new() ];
        todo.insert(i);
        let mut area = 0;
        let mut test_pos;
        while todo.len() > 0 {
            let akt_pos = todo.iter().next().unwrap().clone();
            todo.remove(&akt_pos);
            let akt_y = akt_pos / width ;
            let akt_x = akt_pos - akt_y * width;
            let xy=vec![akt_x, akt_y];
            area += 1;
            for (d,off) in offs.iter().enumerate() {
                test_pos = (akt_pos as i32 + off) as usize;
                if &grid[test_pos..test_pos + 1] == c {
                    todo.insert(test_pos);
                    continue;
                }
                if &grid[test_pos..test_pos + 1] == c_lower {
                    continue;
                }
                borders[d].entry(xy[1 - (d % 2)]).or_insert(vec![]).push((xy[d % 2], xy[d % 2]));

            }
            grid.replace_range(akt_pos..akt_pos + 1, &c_lower);
        }
        for mut dir in borders {
            let keys: Vec<_> =  dir.keys().cloned().collect();
            for row in keys {
                dir.entry(row).and_modify(|m|m.sort_by(|(a,_),(b,_)| a.cmp(b) ));
                let lines = dir.entry(row).or_insert(vec![]);

                let mut min1:usize;
                let mut max1:usize;
                let mut min2:usize;
                let mut max2:usize;
                let mut changed = true;
                while changed {
                    changed = false;
                    let mut p = 0;
                    while p < lines.len() - 1{
                        (min1,max1) = lines[p];
                        (min2,max2) = lines[p+1];
                        if min2 - max1 == 1 {
                            lines[p] = (min1, max2);
                            lines.remove(p + 1);
                            changed = true;
                        }
                        p += 1;
                    }
                }
                result += area * lines.len() as i32;
            }
        }
    }
    result
}


