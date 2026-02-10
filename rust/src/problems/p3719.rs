pub struct Solution;

use std::collections::HashMap;

#[derive(Default, Clone, Copy)]
struct Node {
    min: i32,
    max: i32,
    lazy: i32,
}

struct SegmentTree {
    nodes: Vec<Node>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            nodes: vec![Node::default(); n * 4],
        }
    }

    fn apply(&mut self, i: usize, lazy: i32) {
        self.nodes[i].min += lazy;
        self.nodes[i].max += lazy;
        self.nodes[i].lazy += lazy;
    }

    fn push_down(&mut self, i: usize) {
        let lazy = self.nodes[i].lazy;
        if lazy == 0 {
            return;
        }
        self.apply(i * 2, lazy);
        self.apply(i * 2 + 1, lazy);
        self.nodes[i].lazy = 0;
    }

    fn update(&mut self, i: usize, l: usize, r: usize, ql: usize, qr: usize, val: i32) {
        if ql <= l && r <= qr {
            self.apply(i, val);
            return;
        }
        self.push_down(i);
        let m = l.midpoint(r);
        if ql <= m {
            self.update(i * 2, l, m, ql, qr, val);
        }
        if m < qr {
            self.update(i * 2 + 1, m + 1, r, ql, qr, val);
        }
        self.nodes[i].min = self.nodes[2 * i].min.min(self.nodes[2 * i + 1].min);
        self.nodes[i].max = self.nodes[2 * i].max.max(self.nodes[2 * i + 1].max);
    }

    fn find_first(
        &mut self,
        i: usize,
        l: usize,
        r: usize,
        ql: usize,
        qr: usize,
        target: i32,
    ) -> usize {
        if qr < l || r < ql || target < self.nodes[i].min || target > self.nodes[i].max {
            return usize::MAX;
        }
        if l == r {
            return l;
        }
        self.push_down(i);
        let m = l.midpoint(r);
        let mut ans = self.find_first(i * 2, l, m, ql, qr, target);
        if ans == usize::MAX {
            ans = self.find_first(i * 2 + 1, m + 1, r, ql, qr, target);
        }
        ans
    }
}

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut t = SegmentTree::new(n + 1);

        let mut m = HashMap::with_capacity(n);

        let (mut ans, mut sum) = (0, 0);

        for (i, &x) in nums.iter().enumerate() {
            let val = if x % 2 == 1 { 1 } else { -1 };
            match m.insert(x, i + 1) {
                Some(j) => t.update(1, 0, n, j, i, -val),
                None => {
                    sum += val;
                    t.update(1, 0, n, i + 1, n, val);
                }
            }
            let l = t.find_first(1, 0, n, 0, i - ans, sum);
            if l < usize::MAX {
                ans = i + 1 - l;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::longest_balanced(vec![2, 5, 4, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::longest_balanced(vec![3, 2, 2, 5, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::longest_balanced(vec![1, 2, 3, 2]));
    }
}
