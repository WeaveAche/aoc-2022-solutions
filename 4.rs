use std::fs;

fn part1(contents : &String) -> u64 {
    let mut ans : u64 = 0;

    for line in contents.lines() {
        let ranges : Vec <&str> = line.split(",").collect();

        let range1 : Vec <u64> = ranges[0].split("-").map(|x| x.parse::<u64>().unwrap()).collect();
        let range2 : Vec <u64> = ranges[1].split("-").map(|x| x.parse::<u64>().unwrap()).collect();

        if (range1[0] <= range2[0]) && (range1[1] >= range2[1]) {
            ans += 1;
        } else if (range1[0] >= range2[0]) && (range1[1] <= range2[1]) {
            ans += 1;
        }
    }

    ans
}

fn part2(contents : &String) -> u64 {
    let mut ans : u64 = 0;

    for line in contents.lines() {
        let ranges : Vec <&str> = line.split(",").collect();

        let range1 : Vec <u64> = ranges[0].split("-").map(|x| x.parse::<u64>().unwrap()).collect();
        let range2 : Vec <u64> = ranges[1].split("-").map(|x| x.parse::<u64>().unwrap()).collect();

        if (range1[0] > range2[1]) || (range2[0] > range1[1]) {
            continue;
        }

        ans += 1
    }

    ans
}

fn main() {
    let contents : String = fs::read_to_string("input")
        .expect("Couldn't read file");


    println!("part1 ans: {}", part1(&contents));
    println!("part2 ans: {}", part2(&contents));
}
