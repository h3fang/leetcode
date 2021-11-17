pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut count = [0; 32];
        for n in nums {
            for (i, c) in count.iter_mut().enumerate() {
                if n & (1 << i) > 0 {
                    *c += 1;
                }
            }
        }
        let mut result = 0;
        for (i, c) in count.iter().enumerate() {
            if c % 3 > 0 {
                result |= 1 << i;
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
        let nums = vec![3, 4, 3, 3];
        assert_eq!(4, Solution::single_number(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![9, 1, 7, 9, 7, 9, 7];
        assert_eq!(1, Solution::single_number(nums));
    }
}
