pub struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let (mut ones, mut max, mut cnt, mut prev_zeros) = (0, 0, 0, i32::MIN);
        let s = s.as_bytes();

        for (i, &b) in s.iter().enumerate() {
            cnt += 1;
            if i + 1 == s.len() || b != s[i + 1] {
                if b == b'1' {
                    ones += cnt;
                } else {
                    max = max.max(prev_zeros + cnt);
                    prev_zeros = cnt;
                }
                cnt = 0;
            }
        }

        ones + max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::max_active_sections_after_trade("01".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::max_active_sections_after_trade("0100".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            7,
            Solution::max_active_sections_after_trade("1000100".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            4,
            Solution::max_active_sections_after_trade("01010".to_string())
        );
    }

    #[test]
    fn case5() {
        assert_eq!(
            2,
            Solution::max_active_sections_after_trade("101".to_string())
        );
    }
}
