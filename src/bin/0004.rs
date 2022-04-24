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

    let a = scan.next::<String>();
    let first = a.chars().next().unwrap().is_ascii_uppercase();
    if a.chars()
        .find(|c| c.is_ascii_uppercase() != first)
        .is_some()
    {
        write!(out, "Mix").ok();
    } else if first {
        write!(out, "All Capital Letter").ok();
    } else {
        write!(out, "All Small Letter").ok();
    }
}
