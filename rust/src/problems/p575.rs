use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let unique = candy_type.iter().collect::<HashSet<_>>();
        unique.len().min(candy_type.len() / 2) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::distribute_candies(vec![1, 1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::distribute_candies(vec![1, 1, 1]));
    }
}
