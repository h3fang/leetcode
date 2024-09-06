pub struct Solution;

impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n > 0 {
            if n % 3 == 2 {
                return false;
            }
            n /= 3;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_powers_of_three(12));
    }

    #[test]
    fn case2() {
        assert!(Solution::check_powers_of_three(91));
    }

    #[test]
    fn case3() {
        assert!(!Solution::check_powers_of_three(21));
    }
}
