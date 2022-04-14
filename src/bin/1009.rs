use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};
fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let pos = [(0, 1), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    let seg = HashMap::from([
        (222, 0),
        (18, 1),
        (188, 2),
        (182, 3),
        (114, 4),
        (230, 5),
        (238, 6),
        (146, 7),
        (254, 8),
        (246, 9),
    ]);
    lines.next();
    // let a: Vec<Vec<char>> = (0..3)
    //     .map(|_| lines.next().unwrap().unwrap().chars().collect())
    //     .collect();
    // let b: Vec<Vec<char>> = (0..3)
    //     .map(|_| lines.next().unwrap().unwrap().chars().collect())
    //     .collect();
    // let mut a_int: u64 = 0;
    // let mut b_int: u64 = 0;
    // for i in (0..a[0].len()).step_by(4) {
    //     let mut bit = 0;
    //     for (j, k) in pos {
    //         if i + k < a[j].len() && a[j][i + k] != ' ' {
    //             bit |= 1;
    //         }
    //         bit <<= 1;
    //     }
    //     a_int += seg[&bit];
    //     a_int *= 10;
    // }
    // for i in (0..b[0].len()).step_by(4) {
    //     let mut bit = 0;
    //     for (j, k) in pos {
    //         if i + k < b[j].len() && b[j][i + k] != ' ' {
    //             bit |= 1;
    //         }
    //         bit <<= 1;
    //     }
    //     b_int += seg[&bit];
    //     b_int *= 10;
    // }
    // print!("{}", (a_int + b_int) / 10);
    let mut fun = || {
        let a: Vec<Vec<char>> = (0..3)
            .map(|_| lines.next().unwrap().unwrap().chars().collect())
            .collect();
        (0..a[0].len()).step_by(4).fold(0_u64, |sm, i| {
            (sm + seg[&pos.iter().fold(0_u8, |cr, &(j, k)| {
                (cr + (i + k < a[j].len() && a[j][i + k] != ' ') as u8) << 1
            })]) * 10
        }) / 10
    };
    print!("{}", fun() + fun());
}
/*
 _     _  _     _  _  _  _  _
| |  | _| _||_||_ |_   ||_||_|
|_|  ||_  _|  | _||_|  ||_| _|

4 3
         _   _
  | |_|  _|  _|
  |   | |_   _|
 _       _
  |   | |_
  |   | |_|
*/
