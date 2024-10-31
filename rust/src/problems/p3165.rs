pub struct Solution;

struct SegmentTree {
    tree: Vec<[u32; 4]>,
}

impl SegmentTree {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut t = Self {
            tree: vec![[0; 4]; 2 << (64 - n.leading_zeros())],
        };
        t.build(1, 0, n - 1, nums);
        t
    }

    fn build(&mut self, i: usize, l: usize, r: usize, nums: &[i32]) {
        if l == r {
            self.tree[i][3] = nums[l].max(0) as u32;
            return;
        }
        let m = (l + r) / 2;
        self.build(i * 2, l, m, nums);
        self.build(i * 2 + 1, m + 1, r, nums);
        self.push_up(i);
    }

    fn push_up(&mut self, i: usize) {
        let (l, r) = (i * 2, i * 2 + 1);
        self.tree[i][0] =
            (self.tree[l][0] + self.tree[r][2]).max(self.tree[l][1] + self.tree[r][0]);
        self.tree[i][1] =
            (self.tree[l][0] + self.tree[r][3]).max(self.tree[l][1] + self.tree[r][1]);
        self.tree[i][2] =
            (self.tree[l][2] + self.tree[r][2]).max(self.tree[l][3] + self.tree[r][0]);
        self.tree[i][3] =
            (self.tree[l][2] + self.tree[r][3]).max(self.tree[l][3] + self.tree[r][1]);
    }

    fn update(&mut self, i: usize, l: usize, r: usize, j: usize, v: i32) {
        if l == r {
            self.tree[i][3] = v.max(0) as u32;
            return;
        }
        let m = (l + r) / 2;
        if j <= m {
            self.update(i * 2, l, m, j, v);
        } else {
            self.update(i * 2 + 1, m + 1, r, j, v);
        }
        self.push_up(i);
    }
}

impl Solution {
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut t = SegmentTree::new(&nums);
        let mut result = 0;
        for q in queries {
            t.update(1, 0, n - 1, q[0] as usize, q[1]);
            result = (result + t.tree[1][3]) % 10_0000_0007;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![3, 5, 9];
        let queries = [[1, -2], [0, -3]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(21, Solution::maximum_sum_subsequence(nums, queries));
    }

    #[test]
    fn case2() {
        let nums = vec![0, -1];
        let queries = [[0, -5]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(0, Solution::maximum_sum_subsequence(nums, queries));
    }
}
