use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let n = s.trim().parse::<usize>().unwrap();
    let vec: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            s = String::new();
            stdin().read_line(&mut s).ok();
            let tem = s
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (tem[0], tem[1])
        })
        .collect();
    println!(
        "{:.3}",
        vec.iter().enumerate().fold(0, |mx, (i, (x1, y1))| {
            mx.max(
                vec.iter()
                    .enumerate()
                    .skip(i + 1)
                    .fold(0, |mx, (i, (x2, y2))| {
                        mx.max(vec.iter().skip(i + 1).fold(0, |mx, (x3, y3)| {
                            mx.max(
                                (x1 * y2 + x2 * y3 + x3 * y1 - y1 * x2 - y2 * x3 - y3 * x1).abs(),
                            )
                        }))
                    }),
            )
        }) as f32
            / 2.0
    );
}
