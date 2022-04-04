pub struct Solution;

impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        fn helper(s: &[u8], m: u8) -> i64 {
            let n = s.len();
            let mut left = vec![0; n];
            let mut right = vec![0; n];
            for i in 1..n {
                left[i] = left[i - 1];
                if s[i - 1] != m {
                    left[i] += 1;
                }
            }
            for i in (0..n - 1).rev() {
                right[i] = right[i + 1];
                if s[i + 1] != m {
                    right[i] += 1;
                }
            }
            let mut result = 0;
            for (i, &c) in s.iter().enumerate() {
                if c == m {
                    result += left[i] * right[i];
                }
            }
            result
        }

        helper(s.as_bytes(), b'1') + helper(s.as_bytes(), b'0')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::number_of_ways("001101".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::number_of_ways("11100".to_string()));
    }
}
