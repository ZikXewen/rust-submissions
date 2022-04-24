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

    let s: String = scan.next();
    let len = s.len();

    write!(out, ".").ok();
    for i in 0..len {
        if i % 3 == 2 {
            write!(out, ".*..").ok();
        } else {
            write!(out, ".#..").ok();
        }
    }
    write!(out, "\n.").ok();
    for i in 0..len {
        if i % 3 == 2 {
            write!(out, "*.*.").ok();
        } else {
            write!(out, "#.#.").ok();
        }
    }
    write!(out, "\n#").ok();
    s.chars().enumerate().for_each(|(i, c)| {
        if i % 3 == 2 || (i % 3 == 1 && i + 1 != len) {
            write!(out, ".{}.*", c).ok();
        } else {
            write!(out, ".{}.#", c).ok();
        }
    });
    write!(out, "\n.").ok();
    for i in 0..len {
        if i % 3 == 2 {
            write!(out, "*.*.").ok();
        } else {
            write!(out, "#.#.").ok();
        }
    }
    write!(out, "\n.").ok();
    for i in 0..len {
        if i % 3 == 2 {
            write!(out, ".*..").ok();
        } else {
            write!(out, ".#..").ok();
        }
    }
}
