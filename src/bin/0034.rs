use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    if let [a, b, c] = s
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()[..]
    {
        for i in 1..=100 {
            if a % i == 0 {
                for j in -100..=100 {
                    if j != 0 && c % j == 0 && i * i * c + j * j * a == i * j * b {
                        println!("{} {} {} {}", i, j, a / i, c / j);
                        std::process::exit(0);
                    }
                }
            }
        }
    }
    println!("No Solution");
}
