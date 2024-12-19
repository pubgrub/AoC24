use file;
use itertools::Itertools;
use std::collections::HashSet;
use std::collections::HashMap;

const TEST:i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}",solve1(&lines));
    println!("Part 2: {}",solve2(&lines));
}

fn solve1(lines:&Vec<String>) -> i32 {
    let mut result:i32 = 0;
    let mut x:i32;
    let mut y:i32;
    let width:i32;
    let height;
    let n_blocks:usize;
    if TEST == 0 {
        width = 73;
        height = 73;
        n_blocks = 1024;
    } else {
        width = 9;
        height = 9;
        n_blocks = 12;
    }
    let mut blocked:HashSet<i32> = HashSet::new();
    for l in &lines[0..n_blocks] {
        (x,y) = l.split(",").map(|s| s.parse::<i32>().unwrap()).collect_tuple().unwrap();
        blocked.insert( (y+1)* width  + x + 1); 
        println!("{}-{}",x,y);
    }
    for i in 0..width {
        blocked.insert(i);
        blocked.insert(width * (height-1) + i);
    }
    for i in 1..height {
        blocked.insert(i * width);
        blocked.insert( (i + 1) * width - 1);
    }
    
    for y in 0..height {
        let mut line = "".to_string();
        for x in 0..width{
            if blocked.contains(&(y * width + x)) {
                line += "#";
            } else {
                line +=".";
            }
        }
        println!("{}",line);
    }
    let offs:Vec<i32> = vec![-width,1,width,-1];
    
    let mut grid:HashMap<i32,i32> = HashMap::new();
    grid.insert(width  + 1,0);
    
    let mut todo:HashSet<i32> = HashSet::new();
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
            if ! grid.contains_key(&next_pos) {
                grid.insert(next_pos, akt_cost + 1);
                todo.insert(next_pos);
            } else {
                if grid.get(&next_pos).unwrap() > &(akt_cost + 1) {
                    grid.insert(next_pos,akt_cost + 1);
                    todo.insert(next_pos);
                }
            }
        }
    }
    println!("{:?}",grid);
    result = *grid.get(&(width * (height - 1) - 2)).unwrap();

    println!("{:?}", blocked);

    result
}

fn solve2(lines:&Vec<String>) -> i32 {
    let mut result:i32 = 0;
    result
}
