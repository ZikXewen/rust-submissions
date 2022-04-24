use std::io::{stdin, BufRead};

fn main() {
    let n = stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<u8>()
        .unwrap();
    print!(
        "{:.6}",
        if n == 1 {
            2_f64
        } else if n == 3 {
            2_f64 + 3_f64.sqrt()
        } else if n % 2 == 0 {
            n as f64
        } else {
            (n - 3) as f64 + 12_f64.sqrt()
        }
    );
}
