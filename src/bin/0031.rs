use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let mut ar = s.split_whitespace().map(|x| x.parse::<u32>().unwrap());
    println!(
        "{}",
        (0..3).fold(0, |sm, _| sm + 31 - ar.next().unwrap().leading_zeros())
    );
}
