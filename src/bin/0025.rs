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
    let a = scan.next::<String>().len();
    let op = scan.next::<char>() == '*';
    let b = scan.next::<String>().len();
    if op {
        write!(out, "1").ok();
        for _ in 2..(a + b) {
            write!(out, "0").ok();
        }
    } else if a == b {
        write!(out, "2").ok();
        for _ in 1..a {
            write!(out, "0").ok();
        }
    } else {
        write!(out, "1").ok();
        for _ in 1..(if a < b { b - a } else { a - b }) {
            write!(out, "0").ok();
        }
        write!(out, "1").ok();
        for _ in 1..std::cmp::min(a, b) {
            write!(out, "0").ok();
        }
    }
}
