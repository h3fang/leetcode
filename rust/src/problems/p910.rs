pub struct Solution;

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        let (min, max) = (nums[0], nums[n - 1]);
        let mut result = max - min;
        for w in nums.windows(2) {
            let d = (max - k).max(w[0] + k) - (min + k).min(w[1] - k);
            result = result.min(d);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::smallest_range_ii(vec![0], 0));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::smallest_range_ii(vec![0, 10], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::smallest_range_ii(vec![1, 3, 6], 3));
    }
}
