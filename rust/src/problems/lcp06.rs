pub struct Solution;

impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        coins.into_iter().map(|c| (c + 1) / 2).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::min_count(vec![4, 2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(8, Solution::min_count(vec![2, 3, 10]));
    }
}
