pub struct Solution;

impl Solution {
    pub fn max_strength(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        nums.sort_unstable();
        let k = nums.partition_point(|&x| x < 0);
        let mut result = nums[..k - k % 2].iter().map(|&x| x as i64).product();
        if k < 2 {
            if nums[n - 1] <= 0 {
                return nums[n - 1] as i64;
            }
        }
        for &x in &nums[k..] {
            if x > 0 {
                result *= x as i64;
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
        assert_eq!(1350, Solution::max_strength(vec![3, -1, -5, 2, 5, -9]));
    }

    #[test]
    fn case2() {
        assert_eq!(20, Solution::max_strength(vec![-4, -5, -4]));
    }
}
