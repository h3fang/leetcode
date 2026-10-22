pub struct Solution;

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        if nums[0] > 0 {
            return 0;
        }

        let mut ans = -1;

        for (i, x) in nums.into_iter().enumerate() {
            if x != i as i32 {
                if ans == -1 {
                    ans = x;
                } else {
                    ans &= x;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::sort_permutation(vec![0, 3, 2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::sort_permutation(vec![0, 1, 3, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::sort_permutation(vec![3, 2, 1, 0]));
    }
}
