pub struct Solution;

struct SegmentTree {
    n: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(v: &[i32]) -> Self {
        let n = v.len().next_power_of_two();
        let mut tree = vec![0; 2 * n];
        tree[n..n + v.len()].copy_from_slice(v);
        for i in (1..n).rev() {
            tree[i] = tree[i * 2].max(tree[i * 2 + 1]);
        }
        Self { n, tree }
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
        let mut st = SegmentTree::new(&baskets);
        let mut ans = 0;
        for f in fruits {
            let i = st.find_and_replace(1, 0, st.n - 1, f);
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
