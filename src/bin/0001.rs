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

    let score: u8 = (0..3).map(|_| scan.next::<u8>()).sum();
    let grade = match score {
        0..=49 => "F",
        50..=54 => "D",
        55..=59 => "D+",
        60..=64 => "C",
        65..=69 => "C+",
        70..=74 => "B",
        75..=79 => "B+",
        _ => "A",
    };
    writeln!(out, "{}", grade).ok();
}
