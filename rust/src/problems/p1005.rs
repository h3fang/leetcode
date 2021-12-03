pub struct Solution;

impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort_unstable();
        let mut min = (0, nums[0].abs());
        let mut sum = 0;
        for (i, n) in nums.iter_mut().enumerate() {
            if *n < 0 && k > 0 {
                *n = -*n;
                k -= 1;
            }
            if n.abs() < min.1 {
                min.0 = i;
                min.1 = n.abs();
            }
            sum += *n;
        }
        if k % 2 == 0 {
            sum
        } else {
            sum - 2 * nums[min.0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![4, 2, 3];
        let k = 1;
        assert_eq!(5, Solution::largest_sum_after_k_negations(nums, k));
    }

    #[test]
    fn case2() {
        let nums = vec![3, -1, 0, 2];
        let k = 3;
        assert_eq!(6, Solution::largest_sum_after_k_negations(nums, k));
    }

    #[test]
    fn case3() {
        let nums = vec![4, -3, -1, 5, -4];
        let k = 2;
        assert_eq!(15, Solution::largest_sum_after_k_negations(nums, k));
    }
}
