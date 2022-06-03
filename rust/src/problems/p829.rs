pub struct Solution;

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut result = 0;
        let mut k = 1;
        let is_valid = |n: i32, k: i32| {
            if k % 2 == 0 {
                n % k != 0 && (2 * n) % k == 0
            } else {
                n % k == 0
            }
        };
        while k * (k + 1) <= 2 * n {
            if is_valid(n, k) {
                result += 1;
            }
            k += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::consecutive_numbers_sum(5));
    }
    #[test]
    fn case2() {
        assert_eq!(4, Solution::consecutive_numbers_sum(15));
    }
}
