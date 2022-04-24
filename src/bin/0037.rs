use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut inp = stdin.lock().lines();
    let n = inp
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let m = inp
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .product::<u32>();
    let k = inp.next().unwrap().unwrap().parse::<u32>().unwrap();
    println!(
        "{}",
        ((0..n).fold(0, |sm, _| {
            sm + inp
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        }) - 1)
            / k
            + 1
            + m
    );
}
