pub struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut right: i32 = nums.iter().sum();
        let (mut left, mut ans) = (0, 0);
        for x in nums {
            if x == 0 {
                if left == right {
                    ans += 2;
                } else if left.abs_diff(right) == 1 {
                    ans += 1;
                }
            } else {
                left += x;
                right -= x;
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
        assert_eq!(2, Solution::count_valid_selections(vec![1, 0, 2, 0, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::count_valid_selections(vec![2, 3, 4, 0, 4, 1, 0])
        );
    }
}
