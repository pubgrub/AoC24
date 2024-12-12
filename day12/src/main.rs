use file;
use std::collections::HashSet;

const TEST:i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}",solve1(&lines));
    println!("Part 2: {}",solve2(&lines));
}

fn solve1(lines:&Vec<String>) -> i32 {
    let mut result:i32 = 0;
    let width = lines[0].len() + 2;
    let mut grid:String = ".".repeat(width);
    for l in lines {
        grid.push('.');
        grid.push_str(l);
        grid.push('.');
    }
    grid.push_str( &".".repeat(width));

    let offs = vec![- (width as i32), 1, width as i32, -1];
    for i in 0..grid.len() {
        let mut todo:HashSet<usize> = HashSet::new();
        let c = &grid[i..i+1];
        if "._".contains( c ) {
            continue;
        }
        todo.insert(i);
        let mut area = 1;
        let mut fence = 0;
        while todo.len() > 0 {
            let akt_pos = todo.iter().next().unwrap().clone();
            todo.remove(&akt_pos);
            for off in &offs {
                
                let test_pos = (akt_pos as i32 + off) as usize;
                let test_field = &grid[test_pos..test_pos+1];
                match test_field {
                    "." => fence += 1,
                    "_" => {},
                    c => {
                        todo.insert(test_pos);
                    },
                    _ => {}


                }
                     
            }
        }
    }

    result
}

fn solve2(lines:&Vec<String>) -> i32 {
    let mut result:i32 = 0;
    result
}
