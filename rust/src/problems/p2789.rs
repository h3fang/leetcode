pub struct Solution;

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        let mut b = *nums.last().unwrap() as i64;
        for a in nums.into_iter().rev().skip(1) {
            let a = a as i64;
            if a <= b {
                b += a;
            } else {
                b = a;
            }
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(21, Solution::max_array_value(vec![2, 3, 7, 9, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(11, Solution::max_array_value(vec![5, 3, 3]));
    }
}
