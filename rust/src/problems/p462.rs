pub struct Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums[nums.len() / 2];
        nums.iter().map(|a| (a - n).abs()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_moves2(vec![1, 2, 3]));
    }
}
