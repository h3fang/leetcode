pub struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let (mut ans, mut c, mut l1, mut l2, mut r) = (0, 0, 0, 0, 0);
        for (i, &x) in nums.iter().enumerate() {
            while nums[l2] < x - k * 2 {
                l2 += 1;
            }
            ans = ans.max(num_operations.min((i + 1 - l2) as i32));
            c += 1;
            if i < n - 1 && x == nums[i + 1] {
                continue;
            }
            while nums[l1] < x - k {
                l1 += 1;
            }
            while r < n && nums[r] <= x + k {
                r += 1;
            }
            ans = ans.max((c + num_operations).min((r - l1) as i32));
            c = 0;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::max_frequency(vec![1, 4, 5], 1, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::max_frequency(vec![5, 11, 20, 20], 5, 1));
    }
}
