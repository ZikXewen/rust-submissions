use std::io::{stdin, BufRead};

fn main() {
    let n: u32 = stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();
    let mut arr = [0; 5];
    for i in 1..=n {
        let mut i = i;
        arr[4] += i / 100;
        i %= 100;
        if i >= 90 {
            arr[2] += 1;
            arr[4] += 1;
            i -= 90;
        }
        if i >= 50 {
            arr[3] += 1;
            i -= 50;
        }
        if i >= 40 {
            arr[2] += 1;
            arr[3] += 1;
            i -= 40;
        }
        arr[2] += i / 10;
        i %= 10;
        if i == 9 {
            arr[0] += 1;
            arr[2] += 1;
            i = 0;
        }
        if i >= 5 {
            arr[1] += 1;
            i -= 5;
        }
        if i == 4 {
            arr[0] += 1;
            arr[1] += 1;
            i = 0;
        }
        arr[0] += i;
    }
    for i in 0..5 {
        print!("{} ", arr[i]);
    }
}
