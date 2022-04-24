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
    let m: usize = scan.next();
    let a: Vec<Vec<i32>> = (0..n)
        .map(|_| (0..m).map(|_| scan.next()).collect())
        .collect();
    a.iter().for_each(|row| {
        row.iter().for_each(|x| {
            write!(out, "{} ", x + scan.next::<i32>()).ok();
        });
        writeln!(out).ok();
    });
}
