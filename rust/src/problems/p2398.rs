pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let n = charge_times.len();
        let mut result = 0;
        let mut q = VecDeque::new();
        let mut left = 0;
        let mut sum = 0;
        for right in 0..n {
            while !q.is_empty() && charge_times[*q.back().unwrap()] <= charge_times[right] {
                q.pop_back();
            }
            q.push_back(right);
            sum += running_costs[right] as i64;
            while !q.is_empty()
                && charge_times[*q.front().unwrap()] as i64 + (right + 1 - left) as i64 * sum
                    > budget
            {
                if left == *q.front().unwrap() {
                    q.pop_front();
                }
                sum -= running_costs[left] as i64;
                left += 1;
            }
            result = result.max(right + 1 - left);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::maximum_robots(vec![3, 6, 1, 3, 4], vec![2, 1, 3, 4, 5], 25)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::maximum_robots(vec![11, 12, 19], vec![10, 8, 7], 19)
        );
    }
}
