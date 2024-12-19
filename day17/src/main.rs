use std::u64::MAX;
use file;

const TEST:i32 = 0;
// Part 2 does not run with test data,
// because the "Program" is different

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}",solve1(&lines));
    println!("Part 2: {}",solve2(&lines));
}

fn solve1(lines:&Vec<String>) -> String {
    let mut reg_a = lines[0].split(": ").last().unwrap().parse::<usize>().unwrap();
    let mut reg_b = lines[1].split(": ").last().unwrap().parse::<usize>().unwrap();
    let mut reg_c = lines[2].split(": ").last().unwrap().parse::<usize>().unwrap();
    let prog:Vec<usize> =lines[4].split(": ")
        .last().unwrap()
        .split(",").map(|s| s.parse::<usize>().unwrap()).collect();    
    let mut ptr:usize = 0;
    let mut out:Vec<usize> = vec![];

    let mut instr:usize;
    let mut oper:usize;
    let mut combo:usize = 0;
    loop {
        if ptr >= prog.len() {
            break;
        }
        instr = prog[ptr];
        oper = prog[ptr + 1];
        if ![1,3,4].contains(&instr) {
            combo =  match oper {
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                _ => oper
            };        
        }
        match instr {
            0 => reg_a = reg_a / ( 2_usize.pow(combo as u32)),
            1 => reg_b ^= oper,
            2 => reg_b = combo % 8,
            3 => if reg_a != 0 { 
                    ptr = oper} 
                else {
                    ptr +=2;
                    },
            4 => reg_b ^= reg_c,
            5 => out.push(combo % 8),
            6 => reg_b = reg_a / ( 2_usize.pow(combo as u32)),
            7 => reg_c = reg_a / ( 2_usize.pow(combo as u32)),
            _ => {}
        }
        if instr != 3 { ptr += 2};
    }
    out.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")
}

fn solve2(lines:&Vec<String>) -> usize {
    let mut prog:Vec<usize> =lines[4].split(": ")
        .last().unwrap()
        .split(",").map(|s| s.parse::<usize>().unwrap()).collect();    
    prog.reverse();
    s2(&prog, 0, 0)
}

fn s2(prog:&Vec<usize>,pos:usize,a:usize) -> usize {
    if pos == prog.len() { return a}

    let mut b;
    let mut c;
    let mut res:usize = MAX as usize;
    for i in 0..=7 {
        let a = a * 8 + i;
        b = i;
        b ^= 1;
        c = a / (2_u32.pow(b as u32)) as usize;
        b ^= 5;
        b ^= c;
        b = b % 8;
        if b == prog[pos] {
            res = res.min(s2(prog,pos + 1,a));
        }
    }
    res
}