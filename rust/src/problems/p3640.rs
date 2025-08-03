pub struct Solution;

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let (mut ans, mut i) = (i64::MIN, 1);
        while i < n - 1 {
            let (mut j, mut sum) = (i, nums[i] as i64);

            // middle
            while j + 1 < n && nums[j] > nums[j + 1] {
                sum += nums[j + 1] as i64;
                j += 1;
            }
            if i == j {
                i += 1;
                continue;
            }

            // left
            let mut l = i - 1;
            if nums[l] >= nums[i] {
                i = j;
                continue;
            }
            sum += nums[l] as i64;
            while l > 0 && nums[l - 1] < nums[l] && nums[l - 1] >= 0 {
                sum += nums[l - 1] as i64;
                l -= 1;
            }

            // right
            let mut r = j + 1;
            if r == n || nums[j] >= nums[r] {
                i = j;
                continue;
            }
            let s1 = nums[r] as i64;
            let mut s = s1;
            while r + 1 < n && nums[r] < nums[r + 1] {
                s += nums[r + 1] as i64;
                r += 1;
            }
            sum += s1.max(s);

            i = r;
            ans = ans.max(sum);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(-4, Solution::max_sum_trionic(vec![0, -2, -1, -3, 0, 2, -1]));
    }

    #[test]
    fn case2() {
        assert_eq!(14, Solution::max_sum_trionic(vec![1, 4, 2, 7]));
    }
}
