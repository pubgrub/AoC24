use file;
use itertools::{enumerate, Itertools};
use std::collections::HashMap;
use std::i32::MAX;

const TEST: i32 = 1;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let num_coords: HashMap<char, (i32, i32)> = HashMap::from([
        ('A', (0, 0)),
        ('0', (1, 0)),
        ('1', (2, 1)),
        ('2', (1, 1)),
        ('3', (0, 1)),
        ('4', (2, 2)),
        ('5', (1, 2)),
        ('6', (0, 2)),
        ('7', (2, 3)),
        ('8', (1, 3)),
        ('9', (0, 3)),
    ]);

    let dir_coords: HashMap<char, (i32, i32)> = HashMap::from([
        ('A', (0, 0)),
        ('L', (2, 1)),
        ('R', (0, 1)),
        ('U', (1, 0)),
        ('D', (1, 1)),
    ]);

    let dir_paths: HashMap<(char, char), Vec<Vec<char>>> = HashMap::from([
        (('A', 'R'), vec![vec!['D']]),
        (('A', 'U'), vec![vec!['L']]),
        (('A', 'D'), vec![vec!['D', 'L'], vec!['L', 'D']]),
        (('A', 'L'), vec![vec!['D', 'L', 'L'], vec!['L', 'D', 'L']]),
        (('U', '>'), vec![vec!['R']]),
        (('U', 'D'), vec![vec!['D']]),
        (('U', 'L'), vec![vec!['D', 'L']]),
        (('U', 'R'), vec![vec!['D', 'R'], vec!['R', 'D']]),
        (('L', 'D'), vec![vec!['R']]),
        (('L', 'U'), vec![vec!['R', 'U']]),
        (('L', 'R'), vec![vec!['R', 'R']]),
        (('L', 'A'), vec![vec!['R', 'R', 'U'], vec!['R', 'U', 'R']]),
        (('R', 'A'), vec![vec!['U']]),
        (('R', 'D'), vec![vec!['L']]),
        (('R', 'L'), vec![vec!['L', 'L']]),
        (('R', 'U'), vec![vec!['L', 'U'], vec!['U', 'L']]),
    ]);

    let mut num_paths: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();

    for n1 in &num_coords {
        for n2 in &num_coords {
            if n1.0 == n2.0 {
                continue;
            }
            let mut steps = vec![];
            let dx = n2.1 .0 - n1.1 .0;
            let dy = n2.1 .1 - n1.1 .1;
            if dx < 0 {
                steps.append(&mut vec!['R'; dx.abs() as usize]);
            }
            if dy > 0 {
                steps.append(&mut vec!['U'; dy as usize]);
            }
            if dy < 0 {
                steps.append(&mut vec!['D'; dy.abs() as usize]);
            }
            if dx > 0 {
                steps.append(&mut vec!['L'; dx as usize]);
            }
            println!("{} -> {}", n1.0, n2.0);
            let permuts = steps.iter().permutations(steps.len()).unique();
            'p: for perm in permuts {
                let mut x = n1.1 .0;
                let mut y = n1.1 .1;

                for p in &perm {
                    match *p {
                        'L' => x += 1,
                        'R' => x -= 1,
                        'U' => y += 1,
                        'D' => y -= 1,
                        _ => (),
                    }
                    if x == 2 && y == 0 {
                        continue 'p;
                    }
                }
                num_paths
                    .entry((*n1.0, *n2.0))
                    .or_default()
                    .push(perm.into_iter().copied().collect());
            }
        }
    }
    println!("{:?}", num_paths);

    let mut min_moves: HashMap<(char, char), i32> = HashMap::new();

    for pair in num_paths {
        let mut min_path = i32::MAX;
        for path in pair.1 {
            let mut primed_path: Vec<char> = vec!['A'];
            primed_path.append(&mut path.clone());
            min_path = min_path.min(dive(0, primed_path, &dir_paths));
        }
    }

    fn dive(lvl: i32, mut seq: Vec<char>, dir_path: &HashMap<(char, char), Vec<Vec<char>>>) -> i32 {
        if lvl == 4 {
            return seq.len() as i32;
        }
        for (i, from) in seq[0..seq.len() - 1].iter().enumerate() {
            let to = seq[i + 1];
        }
        0
    }

    let mut d2_commands: HashMap<(char, char), Vec<char>> = HashMap::new();

    for d_from in &dir_coords {
        for d_to in &dir_coords {
            let mut command: Vec<char> = vec![];

            let dx = d_to.1 .0 - d_from.1 .0;
            let dy = d_to.1 .1 - d_from.1 .1;
            if dx < 0 {
                command.append(&mut vec!['R'; dx.abs() as usize]);
            }
            if dy > 0 {
                command.append(&mut vec!['D'; dy as usize]);
            }
            if dy < 0 {
                command.append(&mut vec!['U'; dy.abs() as usize]);
            }
            if dx > 0 {
                command.append(&mut vec!['L'; dx as usize]);
            }
            command.push('A');
            d2_commands.insert((*d_from.0, *d_to.0), command);
        }
    }
    //   println!("{:?}", d2_commands);

    for l in lines {
        let mut act_n_coord = (0, 0);
        let mut n_command: Vec<char> = vec![];
        n_command.push('A');

        for c in l.chars() {
            let target_n_coord = num_coords.get(&c).unwrap();
            let dx = target_n_coord.0 - act_n_coord.0;
            let dy = target_n_coord.1 - act_n_coord.1;
            if dy > 0 {
                n_command.append(&mut vec!['U'; dy as usize]);
            }
            if dx < 0 {
                n_command.append(&mut vec!['R'; dx.abs() as usize]);
            }
            if dx > 0 {
                n_command.append(&mut vec!['L'; dx as usize]);
            }
            if dy < 0 {
                n_command.append(&mut vec!['D'; dy.abs() as usize]);
            }
            n_command.push('A');
            act_n_coord = target_n_coord.clone();
        }
        //println!("{:?}", n_command);
        let mut n2_command = vec![];
        n2_command.push('A');
        for (i, c) in n_command.iter().enumerate() {
            if i < n_command.len() - 1 {
                n2_command.append(&mut d2_commands.get(&(*c, n_command[i + 1])).unwrap().clone());
            }
        }
        //println!("{:?}", n2_command);
        let mut n3_command = vec![];
        for (i, c) in n2_command.iter().enumerate() {
            if i < n2_command.len() - 1 {
                n3_command.append(&mut d2_commands.get(&(*c, n2_command[i + 1])).unwrap().clone());
            }
        }
        //println!("{:?}", n3_command);
        let n3_string: String = n3_command.iter().collect();
        println!("{:?}", n3_string);
    }

    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    result
}
