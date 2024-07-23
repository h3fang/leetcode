pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        let mut m = nums.into_iter().collect::<HashSet<_>>();
        for (x, y) in move_from.into_iter().zip(move_to) {
            m.remove(&x);
            m.insert(y);
        }
        let mut r = m.into_iter().collect::<Vec<_>>();
        r.sort_unstable();
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![5, 6, 8, 9],
            Solution::relocate_marbles(vec![1, 6, 7, 8], vec![1, 7, 2], vec![2, 9, 5])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![2],
            Solution::relocate_marbles(vec![1, 1, 3, 3], vec![1, 3], vec![2, 2])
        );
    }
}
