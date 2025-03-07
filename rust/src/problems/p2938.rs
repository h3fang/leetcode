pub struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let (mut result, mut ones) = (0, 0);
        for b in s.bytes() {
            if b == b'0' {
                result += ones;
            } else {
                ones += 1;
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
        assert_eq!(1, Solution::minimum_steps("101".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_steps("100".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::minimum_steps("0111".to_string()));
    }
}
