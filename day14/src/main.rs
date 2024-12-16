use file;
use regex::Regex;
use itertools::Itertools;


const TEST:i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}",solve1(&lines));
    println!("Part 2: {}",solve2(&lines));
}

fn solve1(lines:&Vec<String>) -> i32 {
    let mut result:i32 = 0;
    let steps = 100;
    let mut max_x = 101;
    let mut max_y = 103;
    
    match TEST {
        1 => {
            max_x = 11;
            max_y = 7;
        },
        _ => {}
    };
    let mid_x = (max_x - 1) / 2;
    let mid_y = (max_y - 1) / 2;
    let re = Regex::new(r"p=(?<x0>\d+),(?<y0>\d+)\sv=(?<dx>-*\d+),(?<dy>-*\d+)").unwrap();
    let mut x0:i32 = 0;
    let mut y0:i32 = 0;
    let mut dx:i32 = 0;
    let mut dy:i32 = 0;
    let mut quadrants:Vec<i32> = vec![0,0,0,0];
    for l in lines {
         (x0,y0,dx,dy) = re.captures(l).map(|caps| {
                let x0 = caps.name("x0").unwrap().as_str().parse().unwrap();
                let y0 = caps.name("y0").unwrap().as_str().parse().unwrap();
                let dx = caps.name("dx").unwrap().as_str().parse().unwrap();
                let dy = caps.name("dy").unwrap().as_str().parse().unwrap();
                [x0,y0,dx,dy]
            }).unwrap().into_iter().collect_tuple().unwrap();
        let final_x = (x0 + dx * steps).rem_euclid(max_x) as i32;
        let final_y = (y0 + dy * steps).rem_euclid(max_y) as i32;
        if final_x == mid_x || final_y == mid_y { continue};
        let mut quadrant = 0;
        if final_x > mid_x { quadrant += 1} ;
        if final_y > mid_y { quadrant += 2} ;
        quadrants[quadrant] += 1;
    }
    result = quadrants.iter().fold(1, |acc,x| acc * x);
    result
}

fn solve2(lines:&Vec<String>) -> i32 {
    let mut result:i32 = 0;
    result
}
