pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut f = vec![0; nums.len()];
        let mut q = VecDeque::new();
        let mut result = i32::MIN;
        for (i, n) in nums.into_iter().enumerate() {
            if !q.is_empty() && i - q.front().unwrap() > k {
                q.pop_front();
            }
            f[i] = q.front().map(|&j| f[j]).unwrap_or(0).max(0) + n;
            result = result.max(f[i]);
            while let Some(&e) = q.back() {
                if f[e] >= f[i] {
                    break;
                }
                q.pop_back();
            }
            q.push_back(i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            37,
            Solution::constrained_subset_sum(vec![10, 2, -10, 5, 20], 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::constrained_subset_sum(vec![-1, -2, -3], 1));
    }

    #[test]
    fn case3() {
        assert_eq!(
            23,
            Solution::constrained_subset_sum(vec![10, -2, -10, -5, 20], 2)
        );
    }
}
