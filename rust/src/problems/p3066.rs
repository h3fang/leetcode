pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut q: BinaryHeap<Reverse<i64>> = nums.into_iter().map(|x| Reverse(x as i64)).collect();
        let mut result = 0;
        while let Some(Reverse(x)) = q.pop() {
            if x >= k {
                break;
            }
            if let Some(Reverse(y)) = q.pop() {
                q.push(Reverse(x * 2 + y));
                result += 1;
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
        assert_eq!(2, Solution::min_operations(vec![2, 11, 10, 1, 3], 10));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::min_operations(vec![1, 1, 2, 4, 9], 20));
    }
}
