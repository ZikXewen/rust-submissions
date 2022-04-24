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

    let ans = (0..5_u8)
        .map(|_| (0..4).map(|_| scan.next::<u8>()).sum::<u8>())
        .enumerate()
        .max_by_key(|(_, x)| *x)
        .unwrap();
    write!(out, "{} {}", ans.0 + 1, ans.1).ok();
}
