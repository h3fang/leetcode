pub struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|mut x| {
                let mut sum = 0;
                while x > 0 {
                    sum += x % 10;
                    x /= 10;
                }
                sum
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_element(vec![10, 12, 13, 14]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::min_element(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(10, Solution::min_element(vec![999, 19, 199]));
    }
}
