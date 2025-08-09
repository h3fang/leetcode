pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), and_values.len());
        let mut f: Vec<Vec<HashMap<i32, i32>>> = vec![vec![HashMap::new(); m]; n];
        f[0][0].insert(nums[0], 0);
        for (i, &x) in nums.iter().enumerate().skip(1) {
            for j in 0..m.min(i + 1) {
                if j > 0
                    && let Some(&y) = f[i - 1][j - 1].get(&and_values[j - 1])
                {
                    f[i][j].insert(x, nums[i - 1] + y);
                }
                for (and, sum) in f[i - 1][j].clone() {
                    let e = f[i][j].entry(and & x).or_insert(i32::MAX);
                    *e = (*e).min(sum);
                }
            }
        }
        if let Some(s) = f[n - 1][m - 1].get(&and_values[m - 1]) {
            s + nums[n - 1]
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            12,
            Solution::minimum_value_sum(vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            17,
            Solution::minimum_value_sum(vec![2, 3, 5, 7, 7, 7, 5], vec![0, 7, 5])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::minimum_value_sum(vec![1, 2, 3, 4], vec![2]));
    }
}
