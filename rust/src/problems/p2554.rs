pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, mut max_sum: i32) -> i32 {
        let banned: HashSet<i32> = banned.into_iter().collect();
        let mut result = 0;
        for i in 1..=n {
            if banned.contains(&i) {
                continue;
            }
            if max_sum >= i {
                max_sum -= i;
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
        assert_eq!(2, Solution::max_count(vec![1, 6, 5], 5, 6));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::max_count(vec![1, 2, 3, 4, 5, 6, 7], 8, 1));
    }

    #[test]
    fn case3() {
        assert_eq!(7, Solution::max_count(vec![11], 7, 50));
    }
}
