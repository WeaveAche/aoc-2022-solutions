use std::fs;
use std::cmp;

fn part1(contents : &String) -> u64 {
    let calories_split = contents.split("\n\n");

    let mut max_calories : u64 = 0;

    for calorie_split in calories_split {
        let mut curr_calorie : u64 = 0;
        for calorie in calorie_split.lines() {
            curr_calorie += calorie.parse::<u64>()
                .expect("Not an integer");
        }

        max_calories = cmp::max(curr_calorie, max_calories);
    }

    max_calories
}

fn part2(contents : &String) -> u64 {
    let calories_split = contents.split("\n\n");

    let mut calorie_count : Vec <u64> = Vec::new();

    for calorie_split in calories_split {
        let mut curr_calorie : u64 = 0;
        for calorie in calorie_split.lines() {
            curr_calorie += calorie.parse::<u64>()
                .expect("Not an integer");
        }

        calorie_count.push(curr_calorie);
    }

    calorie_count.sort();

    (&calorie_count[calorie_count.len() - 3..calorie_count.len()]).iter().sum::<u64>()
}

fn main() {
    let contents : String = fs::read_to_string("input")
        .expect("Couldn't read file");


    println!("part1 ans: {}", part1(&contents));
    println!("part2 ans: {}", part2(&contents));
}
