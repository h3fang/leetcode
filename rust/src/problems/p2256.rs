pub struct Solution;

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut pre = vec![0; n + 1];
        let mut suf = vec![0; n + 1];
        for i in 0..n {
            pre[i + 1] = pre[i] + nums[i] as i64;
        }

        for i in (0..n).rev() {
            suf[i] = suf[i + 1] + nums[i] as i64;
        }
        let mut result = 0;
        let mut min = i64::MAX;
        for i in 0..n {
            let mut avg = pre[i + 1] / (i as i64 + 1);
            if n > i + 1 {
                avg = (avg - suf[i + 1] / (n - i - 1) as i64).abs();
            }
            if avg < min {
                min = avg;
                result = i;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::minimum_average_difference(vec![0]));
    }

    #[test]
    fn case4() {
        assert_eq!(2, Solution::minimum_average_difference(vec![4, 2, 0]));
    }
}
