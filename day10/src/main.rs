use file;

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
    let offs:Vec<i32> = vec![-(width as i32), 1, width as i32, -1];
    let mut levels:Vec<i32> = vec![-1;width];
    for line in lines {
        levels.push(-1);
        levels.append(&mut line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect());
        levels.push(-1);
    }
    levels.append(&mut vec![-1;width]);

    for i in 0..levels.len() {
        if levels[i] == 0 {
            let nines:Vec<usize> = vec![];
            result += climb1(i, &levels, &offs, nines).len() as i32;
        }
    }
    result
}

fn climb1
(pos:usize, levels:&Vec<i32>, offs:&Vec<i32>,mut nines:Vec<usize>) -> Vec<usize>{
    let level = levels[pos];
    if level ==9 {
        if !nines.contains(&pos){
            nines.push(pos);
        }
        return nines;
    }
    for dir in offs {
        let new_pos = (pos as i32 + dir) as usize;
        if levels[new_pos] == level + 1 {
            nines = climb1(new_pos, levels, offs, nines);
        }
    }
nines
}

fn solve2(lines:&Vec<String>) -> i32 {
    let mut result:i32 = 0;

    let width = lines[0].len() + 2;
    let offs:Vec<i32> = vec![-(width as i32), 1, width as i32, -1];
    let mut levels:Vec<i32> = vec![-1;width];
    for line in lines {
        levels.push(-1);
        levels.append(&mut line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect());
        levels.push(-1);
    }
    levels.append(&mut vec![-1;width]);

    for i in 0..levels.len() {
        if levels[i] == 0 {
            let nines:i32= 0;
            result += climb2(i, &levels, &offs, nines);
        }
    }
    result
}

fn climb2
(pos:usize, levels:&Vec<i32>, offs:&Vec<i32>,mut nines:i32) -> i32{
    let level = levels[pos];
    if level ==9 {
        return nines + 1;
    }
    for dir in offs {
        let new_pos = (pos as i32 + dir) as usize;
        if levels[new_pos] == level + 1 {
            nines = climb2(new_pos, levels, offs, nines);
        }
    }
nines
}
