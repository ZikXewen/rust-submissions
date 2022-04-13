#![allow(dead_code)]

#[derive(Debug)]
struct Fenwick {
    tree: Vec<u8>,
    size: usize,
}
impl Fenwick {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size],
            size,
        }
    }
    fn from(ar: &[u8]) -> Self {
        let size = ar.len();
        let mut tree = ar.to_vec();
        for i in 1..size {
            let j = i + (1 << i.trailing_zeros());
            if j <= size {
                tree[j] += tree[i];
            }
        }
        Self { tree, size }
    }
    fn insert(&mut self, mut pos: usize, val: u8) {
        while pos <= self.size {
            self.tree[pos] += val;
            pos += 1 << pos.trailing_zeros();
        }
    }
    fn get(&self, mut pos: usize) -> u8 {
        let mut ret = 0;
        while pos > 0 {
            ret += self.tree[pos];
            pos -= 1 << pos.trailing_zeros();
        }
        ret
    }
}
fn main() {
    let ar: Vec<u8> = (0..13).collect();
    let mut tr = Fenwick::from(&ar);
    tr.insert(1, 10);
    dbg!(tr.get(10));
}
