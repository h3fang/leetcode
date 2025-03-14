pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1) == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_power_of_two(1));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_power_of_two(2));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_power_of_two(3));
    }

    #[test]
    fn case4() {
        assert!(!Solution::is_power_of_two(-4));
    }

    #[test]
    fn case5() {
        assert!(!Solution::is_power_of_two(i32::MAX));
    }

    #[test]
    fn case6() {
        assert!(!Solution::is_power_of_two(i32::MIN));
    }
}
