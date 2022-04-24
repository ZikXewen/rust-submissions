use std::io::{stdin, stdout, BufWriter, Write};
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}
fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let a: Vec<u16> = (0..9).map(|_| scan.next()).collect();
    let sum: u16 = a.iter().sum();

    for i in 1..9 {
        for j in 0..i {
            if sum - a[i] - a[j] == 100 {
                a.iter().enumerate().for_each(|(k, x)| {
                    if k != i && k != j {
                        writeln!(out, "{}", x).ok();
                    }
                });
                return;
            }
        }
    }
}
