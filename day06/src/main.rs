use file;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 1;
    let width = lines[0].len() + 2;
    let offs: Vec<i32> = vec![-(width as i32), 1, width as i32, -1];
    let mut grid = "*".repeat(width);
    for l in lines {
        grid = grid + "*" + l + "*";
    }
    grid += &"*".repeat(width);
    let mut pos = grid.find("^").unwrap();
    grid.replace_range(pos..pos + 1, " ");
    let mut dir = 0;
    let mut nextpos;
    while &grid[pos..pos + 1] != "*" {
        nextpos = (pos as i32 + offs[dir]) as usize;
        while &grid[nextpos..nextpos + 1] == "#" {
            dir = (dir + 1) % 4;
            nextpos = (pos as i32 + offs[dir]) as usize;
        }
        pos = nextpos;
        if &grid[pos..pos + 1] == "." {
            result += 1;
            grid.replace_range(pos..pos + 1, " ");
        }
    }
    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let width = lines[0].len() + 2;
    let offs: Vec<i32> = vec![-(width as i32), 1, width as i32, -1];
    /*
       values in grid:
       20: emtpy, not visited
       21: empty, visited
       99: out of bounds
       0-15: obstacle, bits visited from dir
    */

    let mut orig_grid: Vec<i32> = vec![99; width];
    let mut start_pos = 0;
    for l in lines {
        orig_grid.push(99);
        for c in l.chars() {
            match c {
                '#' => orig_grid.push(0),
                '.' => orig_grid.push(20),
                '^' => {
                    orig_grid.push(21);
                    start_pos = orig_grid.len() - 1;
                }
                _ => {}
            }
        }
        orig_grid.push(99);
    }
    orig_grid.append(&mut vec![99; width]);

    let mut grid = orig_grid.clone();
    let mut possible_obstacles: Vec<usize> = vec![];
    let mut pos = start_pos;
    let mut nextpos;
    let mut dir = 0;
    while grid[pos] != 99 {
        nextpos = (pos as i32 + offs[dir]) as usize;
        while grid[nextpos] < 16 {
            dir = (dir + 1) % 4;
            nextpos = (pos as i32 + offs[dir]) as usize;
        }
        pos = nextpos;
        if grid[pos] == 20 {
            grid[pos] = 21;
            if !possible_obstacles.contains(&pos) {
                possible_obstacles.push(pos)
            };
        }
    }

    let mut dir;
    let mut nextpos;
    'try_obstacle: for new_obstacle in possible_obstacles {
        let mut grid = orig_grid.clone();
        dir = 0;
        pos = start_pos;
        grid[new_obstacle] = 0;
        while grid[pos] != 99 {
            nextpos = (pos as i32 + offs[dir]) as usize;
            if grid[nextpos] < 16 {
                if grid[nextpos] & usize::pow(2, dir as u32) as i32
                    == usize::pow(2, dir as u32) as i32
                {
                    result += 1;
                    continue 'try_obstacle;
                } else {
                    grid[nextpos] = grid[nextpos] | usize::pow(2, dir as u32) as i32;
                }
            }
            while grid[nextpos] < 16 {
                dir = (dir + 1) % 4;
                nextpos = (pos as i32 + offs[dir]) as usize;
            }
            pos = nextpos;
        }
    }
    result
}
