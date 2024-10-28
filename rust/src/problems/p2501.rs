pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let set: HashSet<i64> = nums.into_iter().map(|x| x as i64).collect();
        let mut result = -1;
        for x in &set {
            let mut y = x * x;
            let mut c = 1;
            while set.contains(&y) {
                c += 1;
                y *= y;
            }
            if c >= 2 {
                result = result.max(c);
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
        assert_eq!(3, Solution::longest_square_streak(vec![4, 3, 6, 16, 8, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::longest_square_streak(vec![2, 3, 5, 6, 7]));
    }
}
