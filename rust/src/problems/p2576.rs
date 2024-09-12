pub struct Solution;

impl Solution {
    pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
        fn check(nums: &[i32], m: usize) -> bool {
            let n = nums.len();
            (0..m).all(|i| nums[i] * 2 <= nums[n - m + i])
        }
        nums.sort_unstable();
        let (mut l, mut r) = (0, nums.len() / 2);
        while l < r {
            let m = (r + l + 1) / 2;
            if check(&nums, m) {
                l = m;
            } else {
                r = m - 1;
            }
        }
        l as i32 * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::max_num_of_marked_indices(vec![3, 5, 2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::max_num_of_marked_indices(vec![9, 2, 5, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::max_num_of_marked_indices(vec![7, 6, 8]));
    }
}
