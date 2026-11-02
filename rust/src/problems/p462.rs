pub struct Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mid = *nums.select_nth_unstable(n / 2).1;
        nums.iter().map(|a| (a - mid).abs()).sum()
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
