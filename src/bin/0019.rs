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
    let n: u8 = scan.next();
    let arr: Vec<(i32, i32)> = (0..n).map(|_| (scan.next(), scan.next())).collect();
    let mut ans = i32::MAX;
    for i in 1..(1 << n) {
        let mut prod = 1;
        let mut sum = 0;
        arr.iter().enumerate().for_each(|(j, (s, b))| {
            if i & (1 << j) != 0 {
                prod *= s;
                sum += b;
            }
        });
        ans = ans.min(i32::abs(prod - sum));
    }
    write!(out, "{}", ans).ok();
}
