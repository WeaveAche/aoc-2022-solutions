use std::fs;
use std::collections::VecDeque;
use std::default::Default;

fn part1(contents : &String, instructions : &String) -> String {
    let mut ans = "".to_string();

    let mut rows = contents.lines().collect::<Vec <&str>>();
    let mut n = rows[0].len();

    let mut stacks : Vec<VecDeque <char>> = vec![Default::default(); (n + 2) / 4];

    for row in rows {
        for i in (1..n).step_by(4) {
            if row.as_bytes()[i] != b' ' {
                stacks[(i - 1) / 4].push_back(row.as_bytes()[i] as char);
            }
        }
    }

    for instruction in instructions.lines().collect::<Vec <&str>>() {
        let mut inst_split = instruction.split(" ").collect::<Vec <&str>>();

        let quantity : usize = inst_split[1].parse::<usize>().unwrap();
        let from : usize = inst_split[3].parse::<usize>().unwrap() - 1;
        let to : usize = inst_split[5].parse::<usize>().unwrap() - 1;

        for _i in 0..quantity {
            let to_move = stacks[from].pop_front().unwrap();
            stacks[to].push_front(to_move);
        }
    }

    for stack in stacks { 
        ans.push(*stack.front().unwrap()); 
    }

    ans
}

fn part2(contents : &String, instructions : &String) -> String {
    let mut ans = "".to_string();

    let mut rows = contents.lines().collect::<Vec <&str>>();
    let mut n = rows[0].len();

    let mut stacks : Vec<VecDeque <char>> = vec![Default::default(); (n + 2) / 4];

    for row in rows {
        for i in (1..n).step_by(4) {
            if row.as_bytes()[i] != b' ' {
                stacks[(i - 1) / 4].push_back(row.as_bytes()[i] as char);
            }
        }
    }

    for instruction in instructions.lines().collect::<Vec <&str>>() {
        let mut inst_split = instruction.split(" ").collect::<Vec <&str>>();

        let quantity : usize = inst_split[1].parse::<usize>().unwrap();
        let from : usize = inst_split[3].parse::<usize>().unwrap() - 1;
        let to : usize = inst_split[5].parse::<usize>().unwrap() - 1;

        let mut vec : VecDeque <char> = VecDeque::new();
        for _i in 0..quantity {
            let to_move = stacks[from].pop_front().unwrap();
            vec.push_back(to_move);
        }

        for _i in 0..quantity {
            let to_move = vec.pop_back().unwrap();
            stacks[to].push_front(to_move);
        }
    }

    for stack in stacks { 
        ans.push(*stack.front().unwrap()); 
    }

    ans
}

fn main() {
    let contents : String = fs::read_to_string("input1")
        .expect("Couldn't read file");

    let instructions : String = fs::read_to_string("input2")
        .expect("Couldn't read file");


    println!("part1 ans: {}", part1(&contents, &instructions));
    println!("part2 ans: {}", part2(&contents, &instructions));
}
