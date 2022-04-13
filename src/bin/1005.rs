use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    lines.next();
    let ar: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut lf = 0;
    let mut sm = 0;
    let mut an = (0, 0, 0);
    for (i, x) in ar.iter().enumerate() {
        sm += x;
        if sm > an.0 {
            an = (sm, lf, i);
        }
        if sm <= 0 {
            lf = i + 1;
            sm = 0;
        }
    }
    if an.0 == 0 {
        print!("Empty sequence");
        return;
    }
    for i in an.1..=an.2 {
        print!("{} ", ar[i]);
    }
    print!("\n{}", an.0);
}
/*
8
4 -6 3 -2 6 -4 -6 6
3
-2 -3 -1
*/
