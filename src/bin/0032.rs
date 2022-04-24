use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout());
    let mut str = String::new();
    let mut ar = [0_u16; 10];
    stdin().read_line(&mut String::new()).ok();
    stdin().read_line(&mut str).ok();
    str.split_whitespace()
        .for_each(|c| ar[c.parse::<usize>().unwrap()] += 1);
    let dig = ar.iter().skip(1).position(|x| *x != 0).unwrap() + 1;
    write!(out, "{}", dig).ok();
    ar[dig] -= 1;
    ar.iter().enumerate().for_each(|(i, x)| {
        for _ in 0..*x {
            write!(out, "{}", i).ok();
        }
    })
}
