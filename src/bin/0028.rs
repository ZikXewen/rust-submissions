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
    let mut name: Vec<(usize, String)> = (0..4).map(|i| (i, scan.next::<String>())).collect();
    let arr: Vec<Vec<i8>> = (0..4)
        .map(|_| (0..4).map(|_| scan.next::<i8>()).collect())
        .collect();
    let mut w = [0_i8; 4];
    let mut d = [0_i8; 4];
    let mut s = [0_i8; 4];
    for i in 0..4 {
        for j in 0..i {
            if arr[i][j] > arr[j][i] {
                w[i] += 3;
            } else if arr[i][j] == arr[j][i] {
                w[i] += 1;
                w[j] += 1;
            } else {
                w[j] += 3;
            }
            d[i] += arr[i][j];
            d[i] -= arr[j][i];
            s[i] += arr[i][j];
        }
    }
    name.sort_by(|&(a, _), &(b, _)| {
        w[b].cmp(&w[a])
            .then_with(|| d[b].cmp(&d[a]))
            .then_with(|| s[b].cmp(&s[a]))
    });
    name.iter().for_each(|(a, b)| {
        writeln!(out, "{} {}", b, w[*a]).ok();
    });
}
