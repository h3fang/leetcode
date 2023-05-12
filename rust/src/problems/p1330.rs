pub struct Solution;

impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut v1 = 0;
        let mut m1 = 0;
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for i in 1..n {
            let (x, y) = (nums[i - 1], nums[i]);
            v1 += (x - y).abs();
            m1 = m1.max((y - nums[0]).abs() - (x - y).abs());
            m1 = m1.max((x - nums[n - 1]).abs() - (x - y).abs());
            min = min.min(x.max(y));
            max = max.max(x.min(y));
        }
        v1 + m1.max((max - min) * 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::max_value_after_reverse(vec![2, 3, 1, 5, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            68,
            Solution::max_value_after_reverse(vec![2, 4, 9, 24, 2, 1, 10])
        );
    }
}
