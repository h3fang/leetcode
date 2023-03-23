pub struct Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        fn check(nums: &[i32]) -> bool {
            let mut nums = nums.to_vec();
            nums.sort_unstable();
            let d = nums[1] - nums[0];
            nums.windows(2).skip(1).all(|w| w[1] - w[0] == d)
        }
        l.into_iter()
            .zip(r)
            .map(|(a, b)| check(&nums[a as usize..=b as usize]))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![true, false, true],
            Solution::check_arithmetic_subarrays(
                vec![4, 6, 5, 9, 3, 7],
                vec![0, 0, 2],
                vec![2, 3, 5]
            )
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![false, true, false, false, true, true],
            Solution::check_arithmetic_subarrays(
                vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
                vec![0, 1, 6, 4, 8, 7],
                vec![4, 4, 9, 7, 9, 10]
            )
        );
    }
}
