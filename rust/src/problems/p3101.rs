pub struct Solution;

impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let (mut pre, mut len, mut result) = (-1, 0, 0);
        for x in nums {
            if x != pre {
                len += 1;
            } else {
                len = 1;
            }
            result += len;
            pre = x;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::count_alternating_subarrays(vec![0, 1, 1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::count_alternating_subarrays(vec![1, 0, 1, 0]));
    }
}
