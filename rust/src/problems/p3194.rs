pub struct Solution;

impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort_unstable();
        let n = nums.len();
        (0..n / 2).map(|i| nums[i] + nums[n - 1 - i]).min().unwrap() as f64 * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5)
    }

    #[test]
    fn case1() {
        assert_close(
            5.5,
            Solution::minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]),
        );
    }

    #[test]
    fn case2() {
        assert_close(5.5, Solution::minimum_average(vec![1, 9, 8, 3, 10, 5]));
    }

    #[test]
    fn case3() {
        assert_close(5.0, Solution::minimum_average(vec![1, 2, 3, 7, 8, 9]));
    }
}
