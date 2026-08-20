pub struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];
        ans[0] = nums[0];
        ans[n - 1] = nums[1];

        let (mut l, mut r) = (0, n - 1);

        for &x in &nums[2..] {
            if ans[l] > ans[r] {
                l += 1;
                ans[l] = x;
            } else {
                r -= 1;
                ans[r] = x;
            }
        }

        ans[r..].reverse();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![2, 3, 1], Solution::result_array(vec![2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![5, 3, 4, 8], Solution::result_array(vec![5, 4, 3, 8]));
    }
}
