pub struct Solution;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let (mut result, mut next) = (0, nums[0]);
        for x in nums {
            if next <= x {
                next = x + 1;
            } else {
                result += next - x;
                next += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_increment_for_unique(vec![1, 2, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            6,
            Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7])
        );
    }
}
