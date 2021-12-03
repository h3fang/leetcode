pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut mm = (nums[0], nums[0]);
        let mut result = mm.0;
        for &n in &nums[1..] {
            mm = (
                n.max((mm.0 * n).max(mm.1 * n)),
                n.min((mm.0 * n).min(mm.1 * n)),
            );
            result = result.max(mm.0);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::max_product(vec![2, 3, -2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(32, Solution::max_product(vec![-2, 1, -1, 4, -2, -2]));
    }
}
