use std::fs;
use std::collections::VecDeque;

fn helper(contents : &String) -> Vec <Vec <(usize, usize, usize, usize, bool)>> {
    let rows : Vec <&str> = contents.lines().collect();
    let (r, c) = (rows.len(), rows[0].len());

    let mut ret : Vec <Vec <(usize, usize, usize, usize, bool)>> = vec![vec![(0, 0, 0, 0, false); c]; r];

    for n in 0..r { 
        let mut stack : VecDeque <usize> = VecDeque::new();

        for m in 0..c {
            let height : isize = (rows[n].as_bytes()[m] - b'0').into();

            loop {
                if stack.is_empty() {
                    stack.push_back(m);
                    ret[n][m].0 = m;
                    ret[n][m].4 = true;
                    break;
                }

                let t : usize = *stack.back().unwrap(); 
                let height_t : isize = (rows[n].as_bytes()[t] - b'0').into();

                if height_t < height {
                    stack.pop_back();
                } else {
                    stack.push_back(m);
                    ret[n][m].0 = m - t;
                    break;
                }

            }
        }
    }

    for n in 0..r { 
        let mut stack : VecDeque <usize> = VecDeque::new();

        for m in (0..c).rev() {
            let height : isize = (rows[n].as_bytes()[m] - b'0').into();

            loop {
                if stack.is_empty() {
                    stack.push_back(m);
                    ret[n][m].1 = c - m - 1;
                    ret[n][m].4 = true;
                    break;
                }

                let t : usize = *stack.back().unwrap(); 
                let height_t : isize = (rows[n].as_bytes()[t] - b'0').into();

                if height_t < height {
                    stack.pop_back();
                } else {
                    stack.push_back(m);
                    ret[n][m].1 = t - m;
                    break;
                }

            }
        }
    }

    for m in 0..c { 
        let mut stack : VecDeque <usize> = VecDeque::new();

        for n in 0..r {
            let height : isize = (rows[n].as_bytes()[m] - b'0').into();

            loop {
                if stack.is_empty() {
                    stack.push_back(n);
                    ret[n][m].2 = n;
                    ret[n][m].4 = true;
                    break;
                }

                let t : usize = *stack.back().unwrap(); 
                let height_t : isize = (rows[t].as_bytes()[m] - b'0').into();

                if height_t < height {
                    stack.pop_back();
                } else {
                    stack.push_back(n);
                    ret[n][m].2 = n - t;
                    break;
                }

            }
        }
    }

    for m in 0..c { 
        let mut stack : VecDeque <usize> = VecDeque::new();

        for n in (0..r).rev() {
            let height : isize = (rows[n].as_bytes()[m] - b'0').into();

            loop {
                if stack.is_empty() {
                    stack.push_back(n);
                    ret[n][m].3 = r - n - 1;
                    ret[n][m].4 = true;
                    break;
                }

                let t : usize = *stack.back().unwrap(); 
                let height_t : isize = (rows[t].as_bytes()[m] - b'0').into();

                if height_t < height {
                    stack.pop_back();
                } else {
                    stack.push_back(n);
                    ret[n][m].3 = t - n;
                    break;
                }

            }
        }
    }

    ret
}

fn part1(contents : &String) -> usize {
    let trees = helper(contents);

    let mut ans = 0;
    for row in trees {
        for col in row {
            if col.4 {
                ans += 1;
            }
        }
    }

    ans
}

fn part2(contents : &String) -> usize {
    let trees = helper(contents);

    let mut ans : usize = 0;
    for row in trees {
        for col in row {
            let poss = col.0 * col.1 * col.2 * col.3;

            if poss > ans {
                ans = poss;
            }
        }
    }

    ans
}

fn main() {
    let contents : String = fs::read_to_string("input")
        .expect("Couldn't read file");


    println!("part1 ans: {}", part1(&contents));
    println!("part2 ans: {}", part2(&contents));
}
