pub struct Solution;

struct SegmentTree {
    k: i32,
    tree: Vec<[i32; 6]>,
}

impl SegmentTree {
    fn new(nums: &[i32], k: i32) -> Self {
        let n = nums.len();
        let mut s = Self {
            k,
            tree: vec![[0; 6]; n * 4],
        };
        s.build(nums, 1, 0, n - 1);
        s
    }

    fn new_node(&self, v: i32) -> [i32; 6] {
        let v = v % self.k;
        let mut a = [0; 6];
        a[5] = v;
        a[v as usize] = 1;
        a
    }

    fn build(&mut self, nums: &[i32], i: usize, l: usize, r: usize) {
        if l == r {
            self.tree[i] = self.new_node(nums[l]);
            return;
        }
        let m = l + (r - l) / 2;
        self.build(nums, i * 2, l, m);
        self.build(nums, i * 2 + 1, m + 1, r);
        self.tree[i] = self.merge(self.tree[i * 2], self.tree[i * 2 + 1]);
    }

    fn merge(&self, mut a: [i32; 6], b: [i32; 6]) -> [i32; 6] {
        for x in 0..self.k {
            a[(a[5] * x % self.k) as usize] += b[x as usize];
        }
        a[5] = a[5] * b[5] % self.k;
        a
    }

    fn query(&mut self, i: usize, l: usize, r: usize, ql: usize, qr: usize) -> [i32; 6] {
        if ql <= l && qr >= r {
            return self.tree[i];
        }
        let m = l + (r - l) / 2;
        if qr <= m {
            return self.query(i * 2, l, m, ql, qr);
        } else if ql > m {
            return self.query(i * 2 + 1, m + 1, r, ql, qr);
        }
        let a = self.query(i * 2, l, m, ql, qr);
        let b = self.query(i * 2 + 1, m + 1, r, ql, qr);
        self.merge(a, b)
    }

    fn update(&mut self, i: usize, l: usize, r: usize, idx: usize, val: i32) {
        if l == r {
            self.tree[i] = self.new_node(val);
            return;
        }
        let m = l + (r - l) / 2;
        if idx <= m {
            self.update(i * 2, l, m, idx, val);
        } else {
            self.update(i * 2 + 1, m + 1, r, idx, val);
        }
        self.tree[i] = self.merge(self.tree[i * 2], self.tree[i * 2 + 1]);
    }
}

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut t = SegmentTree::new(&nums, k);
        queries
            .into_iter()
            .map(|q| {
                t.update(1, 0, n - 1, q[0] as usize, q[1]);
                t.query(1, 0, n - 1, q[2] as usize, n - 1)[q[3] as usize]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[2, 2, 0, 2], [3, 3, 3, 0], [0, 1, 0, 1]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![2, 2, 2],
            Solution::result_array(vec![1, 2, 3, 4, 5], 3, queries)
        );
    }

    #[test]
    fn case2() {
        let queries = [[0, 2, 0, 2], [0, 2, 0, 1]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![1, 0],
            Solution::result_array(vec![1, 2, 4, 8, 16, 32], 4, queries)
        );
    }

    #[test]
    fn case3() {
        let queries = [[2, 1, 0, 1]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![5],
            Solution::result_array(vec![1, 1, 2, 1, 1], 2, queries)
        );
    }
}
