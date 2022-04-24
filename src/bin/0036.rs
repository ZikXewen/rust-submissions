use std::io::{stdin, BufRead};

fn main() {
    let n: u128 = stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();
    println!(
        "{}",
        (n / 2 + 1..=n).product::<u128>() / (1..=(n + 1) / 2).product::<u128>() * (n % 2 + 1)
    );
}
