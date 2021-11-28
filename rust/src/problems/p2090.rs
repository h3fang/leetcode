pub struct Solution;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return nums;
        }
        let n = nums.len();
        let mut result = vec![-1; n];
        let k = k as usize;
        let m = 2 * k + 1;
        if m > n {
            return result;
        }

        let mut sum: usize = nums[..m].iter().map(|s| *s as usize).sum();
        result[k] = (sum / m) as i32;

        for i in k + 1..n - k {
            sum -= nums[i - k - 1] as usize;
            sum += nums[i + k] as usize;
            result[i] = (sum / m) as i32;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1],
            Solution::get_averages(vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3)
        );
    }
}
