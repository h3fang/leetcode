pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut ids = (0..n).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|&i| -efficiency[i]);
        let mut q = BinaryHeap::new();
        let mut sum = 0;
        let mut result = 0;
        for i in ids {
            sum += speed[i] as i64;
            q.push(Reverse(speed[i]));
            if q.len() > k {
                let min = q.pop().unwrap();
                sum -= min.0 as i64;
            }
            result = result.max(sum * efficiency[i] as i64);
        }
        (result % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 2;
        assert_eq!(60, Solution::max_performance(n, speed, efficiency, k));
    }

    #[test]
    fn case2() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 3;
        assert_eq!(68, Solution::max_performance(n, speed, efficiency, k));
    }

    #[test]
    fn case3() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 4;
        assert_eq!(72, Solution::max_performance(n, speed, efficiency, k));
    }
}
