pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = nums.len() - k as usize + 1;
        let mut q = BinaryHeap::new();
        for n in nums {
            q.push(n);
            if q.len() > k {
                q.pop();
            }
        }
        q.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)
        );
    }
}
