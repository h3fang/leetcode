pub struct Solution;

use std::collections::BTreeSet;

struct Bit {
    tree: Vec<i32>,
}

impl Bit {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut i: i32, v: i32) {
        let n = self.tree.len() as i32;
        while i < n {
            self.tree[i as usize] = self.tree[i as usize].max(v);
            i += i & (-i);
        }
    }

    fn query(&self, mut i: i32) -> i32 {
        let mut r = 0;
        while i > 0 {
            r = r.max(self.tree[i as usize]);
            i &= i - 1;
        }
        r
    }
}

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut m = 0;
        let mut set = BTreeSet::new();
        set.insert(0);
        for q in &queries {
            m = m.max(q[1]);
            if q[0] == 1 {
                set.insert(q[1]);
            }
        }
        m += 1;
        set.insert(m);

        let mut bit = Bit::new(m as usize);

        let mut pre = 0;
        for &x in &set {
            if x == 0 {
                continue;
            }
            bit.update(x, x - pre);
            pre = x;
        }

        let mut ans = Vec::with_capacity(queries.len());
        for q in queries.iter().rev() {
            let pre = *set.range(..q[1]).next_back().unwrap();
            if q[0] == 1 {
                let next = *set.range(q[1] + 1..).next().unwrap();
                set.remove(&q[1]);
                bit.update(next, next - pre);
            } else {
                let max = bit.query(pre).max(q[1] - pre);
                ans.push(max >= q[2]);
            }
        }
        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = vec![vec![1, 2], vec![2, 3, 3], vec![2, 3, 1], vec![2, 2, 2]];
        assert_eq!(vec![false, true, true], Solution::get_results(queries));
    }

    #[test]
    fn case2() {
        let queries = vec![
            vec![1, 7],
            vec![2, 7, 6],
            vec![1, 2],
            vec![2, 7, 5],
            vec![2, 7, 6],
        ];
        assert_eq!(vec![true, true, false], Solution::get_results(queries));
    }

    #[test]
    fn case3() {
        let queries = vec![
            vec![1, 1],
            vec![1, 11],
            vec![1, 4],
            vec![1, 8],
            vec![2, 13, 7],
        ];
        assert_eq!(vec![false], Solution::get_results(queries));
    }
}
