use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut ar = [0_u16; 10001];
    let mut out = BufWriter::new(stdout());
    stdin().read_line(&mut String::new()).ok();
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.split_whitespace()
        .for_each(|x| ar[x.parse::<usize>().unwrap()] += 1);
    let mx = *ar.iter().max().unwrap();
    ar.iter().enumerate().for_each(|(i, x)| {
        if mx == *x {
            write!(out, "{} ", i).ok();
        }
    })
}
