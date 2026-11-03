pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        nums.into_iter().map(|x| max - x).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::min_moves(vec![2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_moves(vec![4, 4, 5]));
    }
}
