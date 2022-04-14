use std::io::stdin;
fn get_line() -> String {
    let mut str = String::new();
    stdin().read_line(&mut str).unwrap();
    str.trim().to_owned()
}
fn main() {
    let n: usize = get_line().parse().unwrap();
    let mut an = vec![0; 256];
    let mut mx = 0;
    for _ in 0..n {
        let ar: Vec<usize> = get_line()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for i in ar[0]..ar[2] {
            an[i] = an[i].max(ar[1]);
        }
        mx = mx.max(ar[2]);
    }
    for i in 1..=mx {
        if an[i] != an[i - 1] {
            print!("{} {} ", i, an[i]);
        }
    }
}
/*
8
1 11 5
2 6 7
12 7 16
14 3 25
19 18 22
3 13 9
23 13 29
24 4 28
*/
