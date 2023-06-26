pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        if k == costs.len() as i32 {
            return costs.into_iter().map(|c| c as i64).sum();
        }
        let candidates = candidates as usize;
        let mut f = BinaryHeap::new();
        let mut b = BinaryHeap::new();
        let mut left = 0;
        let mut right = costs.len() as i32 - 1;
        let mut result = 0;
        for _ in 0..k {
            while f.len() < candidates && left <= right {
                f.push(Reverse(costs[left as usize]));
                left += 1;
            }
            while b.len() < candidates && left <= right {
                b.push(Reverse(costs[right as usize]));
                right -= 1;
            }
            let x = f.peek().map(|e| e.0).unwrap_or(i32::MAX);
            let y = b.peek().map(|e| e.0).unwrap_or(i32::MAX);
            if x <= y {
                result += x as i64;
                f.pop();
            } else {
                result += y as i64;
                b.pop();
            }
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
            11,
            Solution::total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::total_cost(vec![1, 2, 4, 1], 3, 3));
    }
}
