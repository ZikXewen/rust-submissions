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

    let n: usize = scan.next();
    let k: u16 = scan.next();

    let mut ar = vec![false; n + 1];
    let mut ct: u16 = 0;

    for i in 2..=n {
        if !ar[i] {
            for j in (i..=n).step_by(i) {
                if !ar[j] {
                    ar[j] = true;
                    ct += 1;
                    if ct == k {
                        write!(out, "{}", j).ok();
                        return;
                    }
                }
            }
        }
    }
}
