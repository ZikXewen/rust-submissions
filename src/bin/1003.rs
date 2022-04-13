use std::{
    io::{stdin, BufRead},
    process::exit,
};

fn main() {
    let n: usize = stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();
    if n < 6 {
        print!("no");
        exit(0);
    }
    let mut arr = vec![false; n + 1];
    for i in (0..=n).step_by(6) {
        for i in (i..=n).step_by(9) {
            for i in (i..=n).step_by(20) {
                arr[i] = true;
            }
        }
    }
    (1..=n).filter(|i| arr[*i]).for_each(|x| println!("{x}"));
}
