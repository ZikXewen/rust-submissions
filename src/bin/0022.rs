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

    let inp: u16 = scan.next();
    let lo = (inp - 1) >> 1;
    let hi = inp >> 1;

    for i in 0..=lo {
        for j in 0..(lo << 1 | 1) {
            if j + i == lo || lo + i == j {
                write!(out, "*").ok();
            } else {
                write!(out, "-").ok();
            }
        }
        writeln!(out).ok();
    }
    for i in (0..hi).rev() {
        for j in 0..(lo << 1 | 1) {
            if j + i == lo || lo + i == j {
                write!(out, "*").ok();
            } else {
                write!(out, "-").ok();
            }
        }
        writeln!(out).ok();
    }
}
