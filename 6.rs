use std::fs;
use std::default::Default;

fn checkarr(arr : &[u64], req_count : usize) -> bool {
    let mut count : usize = 0;
    for i in arr {
        match i {
            0 => continue,
            1 => count += 1,
            _ => {
                return false
                }
            }
    }

    if count == req_count {
        return true;
    }

    return false
}

fn part1(contents : &String) -> usize {
    let mut arr : [u64; 26] = Default::default();
    
    for i in 0..4 {
        arr[(contents.as_bytes()[i] as usize) - ('a' as usize)] += 1;
    }
    
    let n : usize = contents.len();

    for i in 4..n {
        if checkarr(&arr, 4) {
            return i;
        } else {
            arr[(contents.as_bytes()[i as usize] as usize) - ('a' as usize)] += 1;
            arr[(contents.as_bytes()[(i - 4) as usize] as usize) - ('a' as usize)] -= 1;
        }
    }
    0
}

fn part2(contents : &String) -> usize {
    let mut arr : [u64; 26] = Default::default();
    
    for i in 0..14 {
        arr[(contents.as_bytes()[i] as usize) - ('a' as usize)] += 1;
    }
    
    let n : usize = contents.len();

    for i in 14..n {
        if checkarr(&arr, 14) {
            return i;
        } else {
            arr[(contents.as_bytes()[i as usize] as usize) - ('a' as usize)] += 1;
            arr[(contents.as_bytes()[(i - 14) as usize] as usize) - ('a' as usize)] -= 1;
        }
    }
    0
}

fn main() {
    let contents : String = fs::read_to_string("input")
        .expect("Couldn't read file");


    println!("part1 ans: {}", part1(&contents));
    println!("part2 ans: {}", part2(&contents));
}
