pub struct Solution;

use std::collections::BinaryHeap;

struct SparseTable {
    min: Vec<Vec<i32>>,
    max: Vec<Vec<i32>>,
}

impl SparseTable {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        let logn = 32 - (n as u32).leading_zeros() as usize;
        let mut min = vec![vec![0; logn]; n];
        let mut max = vec![vec![0; logn]; n];
        for i in 0..n {
            max[i][0] = nums[i];
            min[i][0] = nums[i];
        }
        for j in 1..logn {
            for i in 0..n {
                if i + (1 << j) > n {
                    break;
                }
                max[i][j] = max[i][j - 1].max(max[i + (1 << (j - 1))][j - 1]);
                min[i][j] = min[i][j - 1].min(min[i + (1 << (j - 1))][j - 1]);
            }
        }

        Self { min, max }
    }

    fn query(&self, l: usize, r: usize) -> i32 {
        let j = 31 - ((r + 1 - l) as u32).leading_zeros() as usize;
        let max = self.max[l][j].max(self.max[r + 1 - (1 << j)][j]);
        let min = self.min[l][j].min(self.min[r + 1 - (1 << j)][j]);
        max - min
    }
}

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let st = SparseTable::new(&nums);
        let mut heap = BinaryHeap::with_capacity(n);
        for l in 0..n {
            let val = st.query(l, n - 1);
            heap.push((val, l, n - 1));
        }
        let mut ans: i64 = 0;
        let mut k = k as usize;
        while k > 0 {
            if let Some((val, l, r)) = heap.pop() {
                ans += val as i64;
                if r > l {
                    let new_val = st.query(l, r - 1);
                    heap.push((new_val, l, r - 1));
                }
            }
            k -= 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::max_total_value(vec![1, 3, 2], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::max_total_value(vec![4, 2, 5, 1], 3));
    }
}
