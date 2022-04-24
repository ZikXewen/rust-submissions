use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let n = lines.next().unwrap().unwrap().parse::<u16>().unwrap();
    let mut ans: Vec<String> = (0..n).map(|_| lines.next().unwrap().unwrap()).collect();
    ans.sort();
    ans.dedup();
    ans.iter().for_each(|x| println!("{}", x));
}
