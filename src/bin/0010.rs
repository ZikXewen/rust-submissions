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

    let mut a = vec![1, 0, 0];
    scan.next::<String>().chars().for_each(|c| {
        let c = c as usize - 'A' as usize;
        a.swap(c, (c + 1) % 3);
    });
    write!(out, "{}", a.iter().position(|i| *i == 1).unwrap() + 1).ok();
}
