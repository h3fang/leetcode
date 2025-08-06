pub struct Solution;

struct SegmentTree {
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(v: &[i32]) -> Self {
        let n = v.len();
        let mut r = Self {
            tree: vec![0; 4 * n],
        };
        r.build(v, 1, 0, n - 1);
        r
    }

    fn build(&mut self, v: &[i32], i: usize, l: usize, r: usize) {
        if l == r {
            self.tree[i] = v[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(v, i * 2, l, m);
        self.build(v, i * 2 + 1, m + 1, r);
        self.tree[i] = self.tree[i * 2].max(self.tree[i * 2 + 1]);
    }

    fn find_and_replace(&mut self, i: usize, l: usize, r: usize, x: i32) -> i32 {
        if self.tree[i] < x {
            return -1;
        }
        if l == r {
            self.tree[i] = -1;
            return l as i32;
        }
        let m = (l + r) / 2;
        let mut j = self.find_and_replace(i * 2, l, m, x);
        if j == -1 {
            j = self.find_and_replace(i * 2 + 1, m + 1, r, x);
        }
        self.tree[i] = self.tree[i * 2].max(self.tree[i * 2 + 1]);
        j
    }
}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let n = baskets.len();
        let mut st = SegmentTree::new(&baskets);
        let mut ans = 0;
        for f in fruits {
            let i = st.find_and_replace(1, 0, n - 1, f);
            if i < 0 {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7])
        );
    }
}
