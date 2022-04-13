struct SegmentTree {
    tree: Vec<u8>,
    size: usize,
}
impl SegmentTree {
    fn new(mut arr: Vec<u8>) -> Self {
        let size = arr.len();
        let mut tree = vec![0; size];
        tree.append(&mut arr);
        for i in (1..size).rev() {
            tree[i] = tree[i << 1] + tree[i << 1 | 1];
        }
        Self { tree, size }
    }
    fn update(&mut self, mut pos: usize, val: u8) {
        pos += self.size;
        self.tree[pos] = val;
        while pos > 1 {
            self.tree[pos >> 1] = self.tree[pos] + self.tree[pos ^ 1];
            pos >>= 1;
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> u8 {
        let mut ret = 0;
        l += self.size;
        r += self.size;
        while l < r {
            if l & 1 == 1 {
                ret += self.tree[l];
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                ret += self.tree[r];
            }
            l >>= 1;
            r >>= 1;
        }
        ret
    }
}
fn main() {
    let ar: Vec<u8> = (0..13).collect();
    let mut tree = SegmentTree::new(ar);
    tree.update(0, 10);
    print!("{}", tree.query(0, 11));
}
