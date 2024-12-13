pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut q = BinaryHeap::with_capacity(nums.len());
        for (i, &x) in nums.iter().enumerate() {
            q.push((Reverse(x), Reverse(i)));
        }
        for _ in 0..k {
            let (Reverse(x), i) = q.pop().unwrap();
            q.push((Reverse(x * multiplier), i));
        }
        for (Reverse(x), Reverse(i)) in q {
            nums[i] = x;
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![8, 4, 6, 5, 6],
            Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![16, 8], Solution::get_final_state(vec![1, 2], 3, 4));
    }
}
