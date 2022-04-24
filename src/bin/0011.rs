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

    let mut ar = vec![false; 42];
    let mut ct: u8 = 0;
    (0..10).for_each(|_| {
        let inp = (scan.next::<u16>() % 42) as usize;
        if !ar[inp] {
            ct += 1;
            ar[inp] = true;
        }
    });
    writeln!(out, "{}", ct).ok();
}
