use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let (a, b) = s.split_at(s.find(" ").unwrap());
    let (a, b): (u16, u16) = (a.trim().parse().unwrap(), b.trim().parse().unwrap());
    println!("{}", if a > b { 2 } else { (b - 1) / a + 1 });
}
