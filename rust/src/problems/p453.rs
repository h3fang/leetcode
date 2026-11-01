pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().copied().unwrap();
        nums.into_iter().map(|x| x - min).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::min_moves(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_moves(vec![1, 1, 1]));
    }
}
