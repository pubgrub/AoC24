use file;

const TEST:i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}",solve1(&lines));
    println!("Part 2: {}",solve2(&lines));
}

fn solve1(lines:&Vec<String>) -> String {
    let mut result:String = "".to_string();

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

fn solve2(lines:&Vec<String>) -> i32 {
    let mut result:i32 = 0;
    let reg_b_0 = lines[1].split(": ").last().unwrap().parse::<usize>().unwrap();
    let reg_c_0 = lines[2].split(": ").last().unwrap().parse::<usize>().unwrap();
    let prog:Vec<usize> =lines[4].split(": ")
        .last().unwrap()
        .split(",").map(|s| s.parse::<usize>().unwrap()).collect();    
    let mut ptr:usize;
    let mut out:Vec<usize> = vec![];

    let mut instr:usize;
    let mut oper:usize;
    let mut combo:usize = 0;
    let mut reg_a_0:usize = 0;

    'areg: loop {
        let mut reg_a = reg_a_0;
        let mut reg_b = reg_b_0;
        let mut reg_c = reg_c_0;
        ptr = 0;
        out.clear();
        'run: loop {
            if ptr >= prog.len() {
                //println!("halt");
                break 'run;
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
                3 => { if reg_a != 0 { 
                        ptr = oper} 
                    else {
                        ptr +=2;
                        }
                    //println!("ptr: {}",ptr)
                    },
                4 => reg_b ^= reg_c,
                5 => {
                        out.push(combo % 8);
                        //println!("{:?}-{:?}-",prog,out);

                        if out.len() > prog.len() || out != prog[0..out.len()] {
                            //println!("{:?}*{:?}*",prog,out);
                            break 'run;
                        }
                    },
                6 => reg_b = reg_a / ( 2_usize.pow(combo as u32)),
                7 => reg_c = reg_a / ( 2_usize.pow(combo as u32)),
                _ => {}
            }
            if instr != 3 { ptr += 2};
        }
        if out == prog { break 'areg};
        reg_a_0 += 1;
        if reg_a_0 % 100000 == 0 {
            println!("{}",reg_a_0);
        }
    }
    reg_a_0 as i32
}
