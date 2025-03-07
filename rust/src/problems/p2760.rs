pub struct Solution;

impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let (mut curr, mut result) = (0, 0);
        for e in nums.into_iter() {
            if e > threshold {
                curr = 0;
                continue;
            }

            if e % 2 == curr % 2 {
                curr += 1;
            } else {
                curr = i32::from(e % 2 == 0);
            }
            result = result.max(curr);
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
            3,
            Solution::longest_alternating_subarray(vec![3, 2, 5, 4], 5)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::longest_alternating_subarray(vec![1, 2], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(
            3,
            Solution::longest_alternating_subarray(vec![2, 3, 4, 5], 4)
        );
    }
}
