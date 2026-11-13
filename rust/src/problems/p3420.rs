pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let (k, n) = (k as i64, nums.len());
        let mut q: VecDeque<(i32, i64)> = VecDeque::with_capacity(n);

        let (mut ans, mut cnt, mut r) = (0, 0, n - 1);

        for (l, &x) in nums.iter().enumerate().rev() {
            let mut size = 1;
            while q.back().is_some_and(|e| x >= e.0) {
                let (v, s) = q.pop_back().unwrap();
                size += s;
                cnt += i64::from(x - v) * s;
            }

            q.push_back((x, size));

            while cnt > k {
                let e = q.front_mut().unwrap();
                cnt -= i64::from(e.0 - nums[r]);
                r -= 1;
                e.1 -= 1;
                if e.1 == 0 {
                    q.pop_front();
                }
            }

            ans += (r - l + 1) as i64;
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
            17,
            Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 2, 4, 4], 7)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            12,
            Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 3, 6], 4)
        );
    }
}
