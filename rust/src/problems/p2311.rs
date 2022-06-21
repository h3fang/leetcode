pub struct Solution;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut result = s.iter().filter(|c| **c == b'0').count() as i32;
        let mut num = 0;
        for (i, &c) in s.iter().rev().enumerate() {
            if c == b'0' {
                continue;
            }
            num |= 1 << i;
            if i >= 30 || num > k {
                break;
            }
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::longest_subsequence("1001010".into(), 5));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::longest_subsequence("00101001".into(), 1));
    }
}
