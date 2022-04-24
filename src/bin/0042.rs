use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let n: u8 = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..n {
        println!(
            "{:.0}",
            lines
                .next()
                .unwrap()
                .unwrap()
                .parse::<f64>()
                .unwrap()
                .exp2()
        );
    }
}
