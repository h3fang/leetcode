pub struct Solution;

impl Solution {
    pub fn sum_digit_differences(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut x = nums[0];
        let mut result = 0;
        while x > 0 {
            let mut f = [0; 10];
            for y in nums.iter_mut() {
                f[(*y % 10) as usize] += 1;
                *y /= 10;
            }
            for e in f {
                result += e * (n - e);
            }
            x /= 10;
        }
        result as i64 / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::sum_digit_differences(vec![13, 23, 12]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::sum_digit_differences(vec![10, 10, 10, 10]));
    }
}
