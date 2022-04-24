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

    let arr = [
        vec!['A', 'B', 'C'],
        vec!['B', 'A', 'B', 'C'],
        vec!['C', 'C', 'A', 'A', 'B', 'B'],
    ];
    let name = ["Adrian", "Bruno", "Goran"];
    let mut ans = [0_u8; 3];

    scan.next::<u8>();
    scan.next::<String>()
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            (0..3).for_each(|j| {
                if arr[j][i % arr[j].len()] == c {
                    ans[j] += 1;
                }
            });
        });
    let mx = ans.iter().max().unwrap();
    writeln!(out, "{}", mx).ok();
    ans.iter().enumerate().for_each(|(i, x)| {
        if x == mx {
            writeln!(out, "{}", name[i]).ok();
        }
    })
}
