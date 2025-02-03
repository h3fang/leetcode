pub struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let (mut result, mut sign, mut c) = (1, 0, 0);
        for w in nums.windows(2) {
            let s = (w[1] - w[0]).signum();
            if s == 0 {
                sign = 0;
                c = 1;
            } else if s != sign {
                sign = s;
                c = 2;
            } else {
                c += 1;
            }
            result = result.max(c);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::longest_monotonic_subarray(vec![3, 2, 1]));
    }
}
