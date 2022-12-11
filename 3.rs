use std::fs;
use std::collections::HashSet;

fn common_char(string1 : &String, string2 : &String) -> char {
    for c in string1.chars() {
        if string2.contains(c) {
            return c;
        }
    }

    ' '
}

fn common_char_3(string1 : String, string2 : String, string3 : String) -> char {
    let set1 : HashSet <char> = string1.chars().collect(); 
    let set2 : HashSet <char> = string2.chars().collect(); 
    let set3 : HashSet <char> = string3.chars().collect(); 

    let first_intersection = set1.intersection(&set2)
        .cloned().collect::<HashSet <char>>();

    let intersection = first_intersection.intersection(&set3);

    match intersection.into_iter().next() {
        Some(c) => *c,
        None => ' '
    }
}

fn get_priority(chr : char) -> u64 {
    if chr.is_uppercase() {
        chr as u64 - 'A' as u64 + 27
    } else {
        chr as u64 - 'a' as u64 + 1
    }
}



fn part1(contents : &String) -> u64 {
    let mut ans : u64 = 0;

    for line in contents.lines() {
        let c : char = common_char(&(&line[..line.len()/2]).to_string(), &(&line[line.len()/2..]).to_string());
        let priority : u64 = get_priority(c);

        ans += priority;
    }

    ans
}

fn part2(contents : &String) -> u64 {
    let mut ans : u64 = 0;

    for chunk in contents.lines().collect::<Vec <&str>>().chunks(3) {
        let c : char = common_char_3(chunk[0].to_string(), chunk[1].to_string(), chunk[2].to_string());
        ans += get_priority(c);
    }

    ans
}

fn main() {
    let contents : String = fs::read_to_string("input")
        .expect("Couldn't read file");


    println!("part1 ans: {}", part1(&contents));
    println!("part2 ans: {}", part2(&contents));
}
