use std::fs;

fn part1(contents : &String) -> u64 {
    let mut score = 0;

    for line in contents.lines() {
        let mut line_split = line.split(" ");
        let game = (line_split.next().unwrap(), line_split.next().unwrap());

        match game {
            ("A", "X") => score += 4,
            ("A", "Y") => score += 8,
            ("A", "Z") => score += 3,
            ("B", "X") => score += 1,
            ("B", "Y") => score += 5,
            ("B", "Z") => score += 9,
            ("C", "X") => score += 7,
            ("C", "Y") => score += 2,
            ("C", "Z") => score += 6,
            (&_, _) => (),
        }
    }

    score
}

fn part2(contents : &String) -> u64 {
    let mut score = 0;

    for line in contents.lines() {
        let mut line_split = line.split(" ");
        let game = (line_split.next().unwrap(), line_split.next().unwrap());

        match game {
            ("A", "X") => score += 3,
            ("A", "Y") => score += 4,
            ("A", "Z") => score += 8,
            ("B", "X") => score += 1,
            ("B", "Y") => score += 5,
            ("B", "Z") => score += 9,
            ("C", "X") => score += 2,
            ("C", "Y") => score += 6,
            ("C", "Z") => score += 7,
            (&_, _) => (),
        }
    }

    score
}

fn main() {
    let contents : String = fs::read_to_string("input")
        .expect("Couldn't read file");


    println!("part1 ans: {}", part1(&contents));
    println!("part2 ans: {}", part2(&contents));
}
