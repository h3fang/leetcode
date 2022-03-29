pub struct Solution;

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        fn max_answers(key: &str, k: i32, ans: u8) -> i32 {
            let key = key.as_bytes();
            let mut result = 0;
            let mut sum = 0;
            let mut left = 0;
            for (i, &c) in key.iter().enumerate() {
                if c != ans {
                    sum += 1;
                }
                while sum > k {
                    if key[left] != ans {
                        sum -= 1;
                    }
                    left += 1;
                }
                result = result.max(i - left + 1);
            }
            result as i32
        }
        max_answers(&answer_key, k, b'T').max(max_answers(&answer_key, k, b'F'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::max_consecutive_answers("TTFF".to_string(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::max_consecutive_answers("TFFT".to_string(), 1));
    }

    #[test]
    fn case3() {
        assert_eq!(
            5,
            Solution::max_consecutive_answers("TTFTTFTT".to_string(), 1)
        );
    }
}
