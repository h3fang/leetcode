pub struct Solution;

use std::collections::VecDeque;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut f = vec![0; n + 1];
        f[0] = 1;
        let (mut l, mut sum) = (0, 1);
        let mut min = VecDeque::with_capacity(n);
        let mut max = VecDeque::with_capacity(n);
        for (r, &y) in nums.iter().enumerate() {
            while min.back().is_some_and(|i| nums[*i] >= y) {
                min.pop_back();
            }
            min.push_back(r);
            while max.back().is_some_and(|i| nums[*i] <= y) {
                max.pop_back();
            }
            max.push_back(r);
            while !min.is_empty() && !max.is_empty() {
                let mn = *min.front().unwrap();
                let mx = *max.front().unwrap();
                if nums[mx] - nums[mn] <= k {
                    break;
                }
                sum = (sum - f[l] + MOD) % MOD;
                if mn == l {
                    min.pop_front();
                }
                if mx == l {
                    max.pop_front();
                }
                l += 1;
            }
            f[r + 1] = sum;
            sum = (sum + f[r + 1]) % MOD;
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::count_partitions(vec![9, 4, 1, 3, 7], 4));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::count_partitions(vec![3, 3, 4], 0));
    }
}
