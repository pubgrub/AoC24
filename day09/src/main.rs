use file;
use std::{collections::HashMap, hash::Hash};

const TEST: i32 = 1;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> usize {
    let mut result: usize = 0;
    let dmap:Vec<usize> = lines[0].chars().map(|s| s.to_digit(10).unwrap() as usize ).collect();
    let mut l_pointer = 0;
    let mut r_pointer = dmap.len() -1;
    let mut d_pointer = 0;
    let mut id = 0;
    let mut blocks;
    let mut free_space:usize = 0;
    let mut blocks_to_move;
    let mut res_str = "".to_string();
    if r_pointer % 2 == 1 {
        r_pointer -= 1;
    }
    let mut remaining_blocks = dmap[r_pointer];

    while r_pointer >= l_pointer {  
        if l_pointer % 2 == 0 {
            id = l_pointer / 2;
            if r_pointer > l_pointer {
                blocks = dmap[l_pointer];
            } else {
                blocks = remaining_blocks;
            }
            result += id * blocks * (blocks-1) / 2 +id *  d_pointer * blocks; 
            res_str.push_str( &id.to_string().repeat(blocks));
            d_pointer += blocks;
            l_pointer +=1;
            free_space = dmap[l_pointer];
        } else {
            id = r_pointer / 2;            
            //while free_space > 0 && r_pointer > l_pointer {

            while free_space > 0 && r_pointer > l_pointer{
                blocks_to_move = free_space.min(remaining_blocks);
                free_space -= blocks_to_move;
                remaining_blocks -= blocks_to_move;
                result += id * blocks_to_move  * (blocks_to_move-1) / 2 + id * d_pointer * blocks_to_move ; 
                res_str.push_str( &id.to_string().repeat(blocks_to_move));
                d_pointer += blocks_to_move;
                if remaining_blocks == 0 {
                    r_pointer -= 2;
                    remaining_blocks = dmap[r_pointer];
                    id = r_pointer / 2;
                }
            }
            l_pointer +=1;

        }

    }

    println!("res_str: {}", res_str);
    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut gaps:HashMap<usize,Vec<usize>> = HashMap::new();
    let mut max_gap:usize=0;
    let dmap:Vec<usize> = lines[0].chars().map(|s| s.to_digit(10).unwrap() as usize ).collect();
    let mut d_pointer = 0;
    let mut file_size;
    let mut files:HashMap<usize,(usize, usize)> = HashMap::new(); // id: (pos,size)
    for i in 0..dmap.len(){
        if i % 2 == 1 {
            gaps.entry(dmap[i]).or_insert(vec![]) .push(d_pointer);
        } else {
            files.insert(i / 2, (d_pointer, dmap[i]));
        }
        max_gap = *gaps.keys().max().unwrap();
        d_pointer += dmap[i];
    }
    println!("{:?}", gaps);
    for i in (0..dmap.len()).step_by(2).rev() {
        let id = i / 2;
        file_size = dmap[i];
        for gap_size in file_size..=max_gap {
            if gaps.contains_key(&gap_size) {
                
                files.insert(id,(gaps.get_mut(&id).unwrap().remove(0),dmap[i]));
                break;
            }
        }

    }

    result
}
