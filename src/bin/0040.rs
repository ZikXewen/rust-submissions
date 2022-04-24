use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let n: u8 = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..n {
        let s = lines.next().unwrap().unwrap();
        println!(
            "{}",
            if s.chars().last().unwrap().to_digit(10).unwrap() % 2 == 1
                || (s.len() == 1 && s.parse::<u8>().unwrap() == 2)
            {
                "T"
            } else {
                "F"
            }
        );
    }
}
