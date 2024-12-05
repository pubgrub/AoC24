use file;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    let result1 = solve1(&lines);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", solve2(&lines) - result1);
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut part = 0;
    let mut false_pairs:Vec<(&str,&str)> = vec![];

    'line: for line in lines{
        if line.len() == 0 {
            part = 1;
            continue;
        }
        if part == 0 {
            let (left, right) =  line.split_once("|").unwrap();
            false_pairs.push((right, left));
        } else  {
            let pages:Vec<&str> = line.split(",").collect();
            for i in 0..pages.len() - 1 {
                for j in i+1..pages.len() {
                    if false_pairs.contains(&(pages[i],pages[j])) {
                        continue 'line;
                    }
                }  
            }
            result += pages[pages.len() / 2].parse::<i32>().unwrap();
        }
    }
    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut part = 0;
    let mut false_pairs:Vec<(&str,&str)> = vec![];

    for line in lines{
        if line.len() == 0 {
            part = 1;
            continue;
        }
        if part == 0 {
            let (left, right) =  line.split_once("|").unwrap();
            false_pairs.push((right, left));
        } else  {
            let mut pages:Vec<&str> = line.split(",").collect();
            let mut changed = true;
            while changed {
                changed = false;
                for i in 0..pages.len() - 1 {
                    for j in i+1..pages.len() {
                        if false_pairs.contains(&(pages[i],pages[j])) {
                            pages.swap(i, j);
                            changed = true;
                        }
                    }  
               }
            }
            result += pages[pages.len() / 2].parse::<i32>().unwrap();
        }
    }
    result
}
