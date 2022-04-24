use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let ans = s.trim().chars().fold((0, 0), |(m3, m5), c| {
        let c = c.to_digit(10).unwrap();
        ((10 * m3 + c) % 3, (10 * m5 + c) % 11)
    });
    println!("{} {}", ans.0, ans.1);
}
