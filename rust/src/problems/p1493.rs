pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        let (mut p0, mut p1) = (0, 0);
        for e in nums {
            if e == 0 {
                p1 = p0;
                p0 = 0;
            } else {
                p0 += 1;
                p1 += 1;
            }
            result = result.max(p1);
        }
        if result == n as i32 {
            result -= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::longest_subarray(vec![1, 1, 0, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            5,
            Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1])
        );
    }
}
