pub struct Solution;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let (mut ans, mut ones) = (0, 0);
        for w in s.as_bytes().windows(2) {
            if w[0] == b'1' {
                ones += 1;
                if w[1] == b'0' {
                    ans += ones;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::max_operations("1001101".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::max_operations("00111".to_string()));
    }
}
