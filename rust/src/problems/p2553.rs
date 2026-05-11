pub struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len() * 6);
        let mut digits = Vec::with_capacity(6);
        for mut x in nums {
            while x > 0 {
                digits.push(x % 10);
                x /= 10;
            }
            ans.extend(digits.drain(..).rev());
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![1, 3, 2, 5, 8, 3, 7, 7],
            Solution::separate_digits(vec![13, 25, 83, 77])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![7, 1, 3, 9],
            Solution::separate_digits(vec![7, 1, 3, 9])
        );
    }
}
