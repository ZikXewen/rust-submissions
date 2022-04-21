use std::io::{stdin, BufRead, Lines, StdinLock};

fn line_to_ints(lines: &mut Lines<StdinLock>) -> Vec<usize> {
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
struct Segment {
    left: usize,
    right: usize,
    states: Vec<usize>,
}
struct Meteor {
    left: usize,
    right: usize,
    samples: usize,
}
struct Tree {
    tree: Vec<usize>,
    size: usize,
}
impl Segment {
    fn new(left: usize, right: usize, states: Vec<usize>) -> Self {
        Self {
            left,
            right,
            states,
        }
    }
}
impl Meteor {
    fn from_vec(vec: &Vec<usize>) -> Self {
        Self {
            left: vec[0],
            right: vec[1],
            samples: vec[2],
        }
    }
    fn new(left: usize, right: usize, samples: usize) -> Self {
        Self {
            left,
            right,
            samples,
        }
    }
}
impl Tree {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size * 2],
            size,
        }
    }
    fn update(
        &mut self,
        Meteor {
            left: mut l,
            right: mut r,
            samples: va,
        }: &Meteor,
    ) {
        if l > r {
            self.update(&Meteor::new(l, self.size, *va));
            self.update(&Meteor::new(1, r, *va));
            return;
        }
        l += self.size - 1;
        r += self.size;
        while l < r {
            if l & 1 == 1 {
                self.tree[l] += va;
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.tree[r] += va;
            }
            l >>= 1;
            r >>= 1;
        }
    }
    fn query(&self, mut pos: usize) -> usize {
        pos += self.size - 1;
        let mut res = 0;
        while pos > 1 {
            res += self.tree[pos];
            pos >>= 1;
        }
        res
    }
}
fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let inp = line_to_ints(&mut lines);
    let n = inp[0];
    let m = inp[1];
    let mut owned_sectors = vec![vec![]; n];
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .enumerate()
        .for_each(|(i, x)| owned_sectors[x - 1].push(i + 1));
    let targets = line_to_ints(&mut lines);
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let meteors: Vec<Meteor> = (0..q)
        .map(|_| Meteor::from_vec(&line_to_ints(&mut lines)))
        .collect();
    let mut ans = vec![0; n];
    let mut iteration = vec![Segment::new(0, q, (0..n).collect())];
    loop {
        let mut found = false;
        let mut new_iteration = vec![]; // Might do fixed sized by Option if TLE
        let mut tree = Tree::new(m);
        for Segment {
            left,
            right,
            states,
        } in iteration
        {
            let mut left_states = vec![];
            let mut right_states = vec![];
            let mid = (left + right) / 2;
            for i in left..=mid {
                if i < q {
                    tree.update(&meteors[i]);
                }
            }
            if left == right {
                for state in states {
                    ans[state] = left;
                }
                continue;
            }
            found = true;
            for state in states {
                let collected_samples = owned_sectors[state]
                    .iter()
                    .fold(0, |sm, &sector| sm + tree.query(sector));
                if collected_samples >= targets[state] {
                    left_states.push(state);
                } else {
                    right_states.push(state);
                }
            }
            for i in mid + 1..=right {
                if i < q {
                    tree.update(&meteors[i]);
                }
            }
            new_iteration.push(Segment::new(left, mid, left_states));
            new_iteration.push(Segment::new(mid + 1, right, right_states));
        }
        if !found {
            break;
        }
        iteration = new_iteration;
    }
    for i in ans {
        if i == q {
            println!("NIE");
        } else {
            println!("{}", i + 1);
        }
    }
}
/*
1 10
1 1 1 1 1 1 1 1 1 1
5
3
1 2 1
2 3 1
10 1 1

3 5
1 3 2 1 3
10 5 7
3
4 2 4
1 3 1
3 5 2
*/
