pub struct Solution;

impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut max = *nums.last().unwrap();
        for n in nums.into_iter().rev().skip(1) {
            let k = (n - 1) / max;
            result += k as i64;
            max = n / (k + 1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::minimum_replacement(vec![3, 9, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::minimum_replacement(vec![1, 2, 3, 4, 5]));
    }
}
